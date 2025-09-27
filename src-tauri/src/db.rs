use dirs;
use regex::Regex;
use rusqlite::Connection;
use rusqlite::Error as DbError;
use std::fs;
use std::path::{Path, PathBuf};

// -----------------------------------------------------------------------------------------------

pub struct Memo {
    pub id: String,
    pub topic_id: String,
    pub timestamp: i64,
    pub content: String,
}

impl Memo {
    pub fn all_by_topic(conn: &Connection, topic_id: &str) -> Result<Vec<Memo>, DbError> {
        let mut memos = vec![];

        let mut stmt =
            conn.prepare("SELECT id, topic_id, timestamp, content FROM memo WHERE topic_id = ?1")?;
        let memo_iter = stmt.query_map([topic_id], |row| {
            Ok(Memo {
                id: row.get(0)?,
                topic_id: row.get(1)?,
                timestamp: row.get(2)?,
                content: row.get(3)?,
            })
        })?;

        for m in memo_iter {
            memos.push(m?);
        }

        Ok(memos)
    }

    pub fn latest_by_topic(conn: &Connection, topic_id: &str) -> Result<Memo, DbError> {
        let memo = conn.query_one(
            "SELECT id, topic_id, timestamp, content FROM memo WHERE topic_id = ?1 ORDER BY timestamp DESC LIMIT 1",
            [topic_id],
            |row| {
                Ok(Memo {
                    id: row.get(0)?,
                    topic_id: row.get(1)?,
                    timestamp: row.get(2)?,
                    content: row.get(3)?,
                })
            }
        )?;

        Ok(memo)
    }

    pub fn create(
        conn: &Connection,
        id: &str,
        topic_id: &str,
        timestamp: i64,
        content: &str,
    ) -> Result<Self, DbError> {
        let _ = conn.execute(
            "INSERT INTO memo (id, topic_id, timestamp, content) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, topic_id, timestamp, content],
        )?;

        Ok(Memo {
            id: id.to_owned(),
            topic_id: topic_id.to_owned(),
            timestamp,
            content: content.to_owned(),
        })
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), DbError> {
        let _ = conn.execute("DELETE FROM memo WHERE id = ?1", [&self.id])?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------------------------

pub struct Topic {
    pub id: String,
    pub title: String,
    pub timestamp: i64,
}

impl Topic {
    pub fn all(conn: &Connection) -> Result<Vec<Topic>, DbError> {
        let mut topics = vec![];

        let mut stmt = conn.prepare("SELECT id, title, timestamp FROM topic")?;
        let topic_iter = stmt.query_map([], |row| {
            Ok(Topic {
                id: row.get(0)?,
                title: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;

        for t in topic_iter {
            topics.push(t?);
        }

        Ok(topics)
    }

    pub fn search(conn: &Connection, keyword: &str) -> Result<Vec<Topic>, DbError> {
        let (words, tags) = Topic::split_keyword(keyword);

        let mut wheres = vec![];

        // TODO: Use placeholder
        if !words.is_empty() {
            let conditions = words
                .iter()
                .map(|w| format!("content like '%{w}%'"))
                .collect::<Vec<String>>();
            let condition = conditions.join(" or ");

            wheres.push(format!(
                "id IN (SELECT DISTINCT topic_id FROM memo WHERE {condition})"
            ));
        }

        // TODO: Use placeholder
        if !tags.is_empty() {
            let conditions = tags
                .iter()
                .map(|t| format!("'{t}'"))
                .collect::<Vec<String>>();
            let condition = conditions.join(", ");

            wheres.push(format!(
                "id IN (SELECT DISTINCT topic_id FROM topic_tag WHERE name IN ({condition}))"
            ));
        }

        let mut query = "SELECT id, title, timestamp FROM topic".to_owned();
        if !wheres.is_empty() {
            let condition = wheres.join(" AND ");
            query.push_str(&format!(" WHERE {condition}"));
        }

        let mut topics = vec![];

        let mut stmt = conn.prepare(&query)?;
        let topic_iter = stmt.query_map([], |row| {
            Ok(Topic {
                id: row.get(0)?,
                title: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;

        for t in topic_iter {
            topics.push(t?);
        }

        Ok(topics)
    }

    pub fn create(
        conn: &Connection,
        id: &str,
        title: &str,
        timestamp: i64,
    ) -> Result<Self, DbError> {
        let _ = conn.execute(
            "INSERT INTO topic (id, title, timestamp) VALUES (?1, ?2, ?3)",
            rusqlite::params![id, title, timestamp],
        )?;

        Ok(Topic {
            id: id.to_owned(),
            title: title.to_owned(),
            timestamp,
        })
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), DbError> {
        let _ = conn.execute("DELETE FROM topic WHERE id = ?1", [&self.id])?;
        Ok(())
    }

    pub fn update(&self, conn: &Connection, title: &str, timestamp: i64) -> Result<Self, DbError> {
        let _ = conn.execute(
            "UPDATE topic SET title = ?2, timestamp = ?3 WHERE id = ?1",
            rusqlite::params![&self.id, title, timestamp],
        )?;
        Ok(Topic {
            id: self.id.clone(),
            title: title.to_owned(),
            timestamp,
        })
    }

    fn split_keyword(keyword: &str) -> (Vec<String>, Vec<String>) {
        let re = Regex::new(r"\s").unwrap();
        let words_and_tags = re.split(keyword).collect::<Vec<&str>>();

        let mut words = vec![];
        let mut tags = vec![];
        for wt in words_and_tags {
            if wt.starts_with('#') && wt.len() > 2 {
                tags.push(wt.trim_start_matches('#').to_owned());
            } else if !wt.starts_with('#') && wt.len() > 1 {
                words.push(wt.to_owned());
            }
        }

        (words, tags)
    }
}

// -----------------------------------------------------------------------------------------------

pub struct TopicTag {
    pub name: String,
    pub topic_id: String,
}

impl TopicTag {
    pub fn all_by_topic(conn: &Connection, topic_id: &str) -> Result<Vec<String>, DbError> {
        let mut names = vec![];

        let mut stmt = conn.prepare("SELECT name FROM topic_tag WHERE topic_id = ?1")?;
        let name_iter = stmt.query_map([topic_id], |row| row.get(0))?;

        for n in name_iter {
            names.push(n?);
        }

        Ok(names)
    }

    pub fn create(conn: &Connection, name: &str, topic_id: &str) -> Result<Self, DbError> {
        let _ = conn.execute(
            "INSERT INTO topic_tag (name, topic_id) VALUES (?1, ?2)",
            [name, topic_id],
        )?;

        Ok(TopicTag {
            name: name.to_owned(),
            topic_id: topic_id.to_owned(),
        })
    }

    pub fn delete(&self, conn: &Connection) -> Result<(), DbError> {
        let _ = conn.execute(
            "DELETE FROM topic_tag WHERE name = ?1 and topic_id = ?2",
            [&self.name, &self.topic_id],
        )?;

        Ok(())
    }
}

// -----------------------------------------------------------------------------------------------

pub fn setup(file_path: Option<&Path>, in_memory: bool) -> Result<Connection, DbError> {
    let db = if in_memory {
        Connection::open_in_memory()
    } else if let Some(db_path) = file_path {
        Connection::open(db_path)
    } else {
        Connection::open(create_default_path())
    }?;

    create_table_if_not_exists(&db)?;

    Ok(db)
}

pub fn create_table_if_not_exists(conn: &Connection) -> Result<(), DbError> {
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS memo (id TEXT, topic_id TEXT, timestamp INTEGER, content TEXT)",
        [],
    )?;
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS topic (id TEXT, timestamp INTEGER, title TEXT)",
        [],
    )?;
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS topic_tag (name TEXT, topic_id TEXT)",
        [],
    )?;
    Ok(())
}

fn create_default_path() -> PathBuf {
    let base = dirs::data_dir().unwrap().join("markdown-memo");

    if !base.exists() {
        fs::create_dir_all(&base).unwrap();
    }

    base.join("memo.db")
}

// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memo_all_by_topic_is_empty() {
        let conn = setup_connect();
        let m = Memo::all_by_topic(&conn, "t1").unwrap();
        assert_eq!(0, m.len());
    }

    #[test]
    fn test_memo_latest_by_topic_is_empty() {
        let conn = setup_connect();
        let e = Memo::latest_by_topic(&conn, "t1").err();
        assert!(e.is_some());
    }

    #[test]
    fn test_memo_create() {
        let conn = setup_connect();
        let m = Memo::create(&conn, "m1", "t1", i64::MAX, "content1").unwrap();
        assert_eq!("m1", &m.id);
        assert_eq!("t1", &m.topic_id);
        assert_eq!(i64::MAX, m.timestamp);
        assert_eq!("content1", &m.content);

        let memos = Memo::all_by_topic(&conn, "t1").unwrap();
        for m in memos {
            assert_eq!("m1", &m.id);
            assert_eq!("t1", &m.topic_id);
            assert_eq!(i64::MAX, m.timestamp);
            assert_eq!("content1", &m.content);
        }

        let m = Memo::latest_by_topic(&conn, "t1").unwrap();
        assert_eq!("m1", &m.id);
        assert_eq!("t1", &m.topic_id);
        assert_eq!(i64::MAX, m.timestamp);
        assert_eq!("content1", &m.content);
    }

    #[test]
    fn test_memo_delete() {
        let conn = setup_connect();
        let m = Memo::create(&conn, "m1", "t1", i64::MAX, "content1").unwrap();

        m.delete(&conn).unwrap();

        let memos = Memo::all_by_topic(&conn, "t1").unwrap();
        assert_eq!(0, memos.len());
    }

    #[test]
    fn test_topic_all_is_empty() {
        let conn = setup_connect();
        let m = Topic::all(&conn).unwrap();
        assert_eq!(0, m.len());
    }

    #[test]
    fn test_topic_search_words() {
        let conn = setup_connect();
        Topic::create(&conn, "t1", "title1", 0).unwrap();
        Topic::create(&conn, "t2", "title2", 1).unwrap();
        Memo::create(&conn, "m1", "t1", 0, "abc").unwrap();
        Memo::create(&conn, "m2", "t1", 0, "cde").unwrap();
        Memo::create(&conn, "m3", "t2", 0, "bcd").unwrap();

        let topics = Topic::search(&conn, "bc").unwrap();
        assert_eq!(2, topics.len());

        let topics = Topic::search(&conn, "de").unwrap();
        assert_eq!(1, topics.len());

        let topics = Topic::search(&conn, "def").unwrap();
        assert_eq!(0, topics.len());
    }

    #[test]
    fn test_topic_search_tags() {
        let conn = setup_connect();
        Topic::create(&conn, "t1", "title1", 0).unwrap();
        Topic::create(&conn, "t2", "title2", 1).unwrap();
        TopicTag::create(&conn, "abc", "t1").unwrap();
        TopicTag::create(&conn, "cde", "t1").unwrap();
        TopicTag::create(&conn, "abc", "t2").unwrap();

        let topics = Topic::search(&conn, "#abc").unwrap();
        assert_eq!(2, topics.len());

        let topics = Topic::search(&conn, "#cde").unwrap();
        assert_eq!(1, topics.len());

        let topics = Topic::search(&conn, "#def").unwrap();
        assert_eq!(0, topics.len());
    }

    #[test]
    fn test_topic_search_words_tags() {
        let conn = setup_connect();
        Topic::create(&conn, "t1", "title1", 0).unwrap();
        Topic::create(&conn, "t2", "title2", 1).unwrap();
        Memo::create(&conn, "m1", "t1", 0, "abc").unwrap();
        Memo::create(&conn, "m2", "t1", 0, "def").unwrap();
        Memo::create(&conn, "m3", "t2", 0, "bcd").unwrap();
        TopicTag::create(&conn, "abc", "t1").unwrap();
        TopicTag::create(&conn, "cde", "t1").unwrap();
        TopicTag::create(&conn, "abc", "t2").unwrap();

        let topics = Topic::search(&conn, "bc #abc").unwrap();
        assert_eq!(2, topics.len());

        let topics = Topic::search(&conn, "de #cde").unwrap();
        assert_eq!(1, topics.len());

        let topics = Topic::search(&conn, "cd #cde").unwrap();
        assert_eq!(0, topics.len());
    }

    #[test]
    fn test_topic_create() {
        let conn = setup_connect();
        let m = Topic::create(&conn, "t1", "title1", i64::MAX).unwrap();
        assert_eq!("t1", &m.id);
        assert_eq!("title1", &m.title);
        assert_eq!(i64::MAX, m.timestamp);

        let topics = Topic::all(&conn).unwrap();
        for t in topics {
            assert_eq!("t1", &t.id);
            assert_eq!("title1", &t.title);
            assert_eq!(i64::MAX, t.timestamp);
        }
    }

    #[test]
    fn test_topic_delete() {
        let conn = setup_connect();
        let m = Topic::create(&conn, "t1", "title1", i64::MAX).unwrap();

        m.delete(&conn).unwrap();

        let topics = Topic::all(&conn).unwrap();
        assert_eq!(0, topics.len());
    }

    #[test]
    fn test_topic_update() {
        let conn = setup_connect();
        let m = Topic::create(&conn, "t1", "title1", i64::MAX).unwrap();

        let u = m.update(&conn, "title2", 1).unwrap();
        assert_eq!("t1", &u.id);
        assert_eq!("title2", &u.title);
        assert_eq!(1, u.timestamp);

        let topics = Topic::all(&conn).unwrap();
        for t in topics {
            assert_eq!("t1", &t.id);
            assert_eq!("title2", &t.title);
            assert_eq!(1, t.timestamp);
        }
    }

    #[test]
    fn test_split_keyword_word() {
        let (w, t) = Topic::split_keyword("a bc  bcd");
        assert_eq!(vec!["bc".to_owned(), "bcd".to_owned()], w);
        assert!(t.is_empty());
    }

    #[test]
    fn test_split_keyword_tag() {
        let (w, t) = Topic::split_keyword("#a　#bc　　#bcd");
        assert!(w.is_empty());
        assert_eq!(vec!["bc".to_owned(), "bcd".to_owned()], t);
    }

    #[test]
    fn test_split_keyword_word_tag() {
        let (w, t) = Topic::split_keyword("a #bc  bcd ");
        assert_eq!(vec!["bcd".to_owned()], w);
        assert_eq!(vec!["bc".to_owned()], t);
    }

    #[test]
    fn test_topic_tag_all_by_topic_is_empty() {
        let conn = setup_connect();
        let m = TopicTag::all_by_topic(&conn, "t1").unwrap();
        assert_eq!(0, m.len());
    }

    #[test]
    fn test_topic_tag_create() {
        let conn = setup_connect();
        let m = TopicTag::create(&conn, "tag", "t1").unwrap();
        assert_eq!("tag", &m.name);
        assert_eq!("t1", &m.topic_id);

        let tags = TopicTag::all_by_topic(&conn, "t1").unwrap();
        for t in tags {
            assert_eq!("tag", &t);
        }
    }

    #[test]
    fn test_topic_tag_delete() {
        let conn = setup_connect();
        let m = TopicTag::create(&conn, "tag", "t1").unwrap();

        m.delete(&conn).unwrap();

        let tags = TopicTag::all_by_topic(&conn, "t1").unwrap();
        assert_eq!(0, tags.len());
    }

    fn setup_connect() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_table_if_not_exists(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_default_path() {
        let path = create_default_path();
        assert!(path.parent().unwrap().exists());

        let path = create_default_path();
        assert!(path.parent().unwrap().exists());

        let parent = path.parent().unwrap();
        let parent_name = parent.file_name().unwrap().to_os_string();
        if parent.is_dir() && parent_name.to_str().unwrap() == "markdown-memo" {
            fs::remove_dir(parent).unwrap();
        }
    }

    #[test]
    fn test_create_table_if_not_exists() {
        let conn = Connection::open_in_memory().unwrap();
        create_table_if_not_exists(&conn).unwrap();
        create_table_if_not_exists(&conn).unwrap();
    }
}
