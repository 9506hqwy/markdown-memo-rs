use crate::error::Error;
use crate::{db, model, AppData};
use std::time::SystemTime;
use uuid::Uuid;

// -----------------------------------------------------------------------------------------------

pub fn create_memo_fn(data: &AppData, topic_id: &str, content: &str) -> Result<model::Memo, Error> {
    let id = Uuid::new_v4().to_string();

    let title = parse_title(content);

    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let timestamp = duration.as_secs() as i64;

    let db = data.db.lock()?;
    if let Some(topic) = db::Topic::all(&db)?.iter().find(|t| t.id == topic_id) {
        // Update exist topic.
        topic.update(&db, &title, timestamp)?;
    } else {
        // Create new topic.
        db::Topic::create(&db, topic_id, &title, timestamp)?;
    }

    let memo = db::Memo::create(&db, &id, topic_id, timestamp, content)?;
    Ok(model::Memo {
        id: memo.id,
        topic_id: memo.topic_id,
        timestamp: memo.timestamp,
        latest: true,
        content: memo.content,
    })
}

pub fn delete_memo_fn(data: &AppData, topic_id: &str, id: &str) -> Result<(), Error> {
    let db = data.db.lock()?;

    let memos = db::Memo::all_by_topic(&db, topic_id)?;
    let mut delete_all = true;
    for memo in memos {
        if memo.id == id {
            memo.delete(&db)?;
        } else {
            delete_all = false;
        }
    }

    if delete_all {
        // Delete related topic.
        let topics = db::Topic::all(&db)?;
        for topic in topics {
            if topic.id == topic_id {
                topic.delete(&db)?;
            }
        }

        // Delete related tags.
        let tags = db::TopicTag::all_by_topic(&db, topic_id)?;
        for tag in tags {
            let t = db::TopicTag {
                name: tag.to_owned(),
                topic_id: topic_id.to_owned(),
            };
            t.delete(&db)?;
        }
    }

    Ok(())
}

pub fn get_memo_fn(data: &AppData, topic_id: &str, id: Option<&str>) -> Result<model::Memo, Error> {
    let db = data.db.lock()?;

    let memos = db::Memo::all_by_topic(&db, topic_id)?;

    if let Ok(latest) = db::Memo::latest_by_topic(&db, topic_id) {
        let memo_id = id.unwrap_or(&latest.id);

        let memo = memos
            .iter()
            .find(|m| m.id == memo_id)
            .ok_or_else(|| Error::NotFound(memo_id.to_owned()))?;

        Ok(model::Memo {
            id: memo.id.clone(),
            topic_id: memo.topic_id.clone(),
            timestamp: memo.timestamp,
            latest: memo.id == latest.id,
            content: memo.content.clone(),
        })
    } else {
        Ok(model::Memo {
            id: "".to_owned(),
            topic_id: topic_id.to_owned(),
            timestamp: 0,
            latest: true,
            content: "".to_owned(),
        })
    }
}

pub fn get_memo_all_fn(data: &AppData, topic_id: &str) -> Result<Vec<model::Memo>, Error> {
    let db = data.db.lock()?;
    let memos = db::Memo::all_by_topic(&db, topic_id)?;

    let mut models = vec![];
    for memo in memos {
        models.push(model::Memo {
            id: memo.id,
            topic_id: memo.topic_id,
            timestamp: memo.timestamp,
            latest: false,
            content: memo.content,
        });
    }

    models.sort_unstable_by_key(|t| t.timestamp);
    models.reverse();

    if !models.is_empty() {
        models[0].latest = true;
    }

    Ok(models)
}

fn parse_title(content: &str) -> String {
    content
        .chars()
        .skip_while(|c| c == &'#')
        .take_while(|c| c != &'\n')
        .take(10)
        .collect::<String>()
        .trim()
        .to_owned()
}

// -----------------------------------------------------------------------------------------------

pub fn get_topics_fn(data: &AppData, keyword: &str) -> Result<Vec<model::Topic>, Error> {
    let db = data.db.lock()?;
    let topics = if keyword.is_empty() {
        db::Topic::all(&db)
    } else {
        db::Topic::search(&db, keyword)
    }?;

    let mut models = vec![];
    for topic in topics {
        models.push(model::Topic {
            id: topic.id,
            title: topic.title,
            timestamp: topic.timestamp,
        });
    }

    models.sort_unstable_by_key(|t| t.timestamp);
    models.reverse();

    Ok(models)
}

// -----------------------------------------------------------------------------------------------

pub fn add_memo_tag_fn(data: &AppData, topic_id: &str, name: &str) -> Result<(), Error> {
    let db = data.db.lock()?;
    db::TopicTag::create(&db, name, topic_id)?;
    Ok(())
}

pub fn remove_memo_tag_fn(data: &AppData, topic_id: &str, name: &str) -> Result<(), Error> {
    let db = data.db.lock()?;
    let m = db::TopicTag {
        name: name.to_owned(),
        topic_id: topic_id.to_owned(),
    };
    m.delete(&db)?;
    Ok(())
}

pub fn get_memo_tag_fn(data: &AppData, topic_id: &str) -> Result<Vec<String>, Error> {
    let db = data.db.lock()?;
    let tags = db::TopicTag::all_by_topic(&db, topic_id)?;
    Ok(tags)
}

// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_table_if_not_exists;
    use rusqlite::Connection;
    use std::sync::Mutex;

    #[test]
    fn test_create_memo_fn_new_topic() {
        let data = setup_appdate();

        let memo = create_memo_fn(&data, "t1", "content1").unwrap();
        assert_ne!("", memo.id);
        assert_eq!("t1", memo.topic_id);
        assert_ne!(0, memo.timestamp);
        assert_eq!("content1", memo.content);

        {
            let conn = data.db.lock().unwrap();

            let memos = db::Memo::all_by_topic(&conn, "t1").unwrap();
            for m in memos {
                assert_ne!("", m.id);
                assert_eq!("t1", m.topic_id);
                assert_ne!(0, m.timestamp);
                assert_eq!("content1", m.content);
            }

            let topics = db::Topic::all(&conn).unwrap();
            let t = topics.iter().find(|t| t.id == "t1").unwrap();
            assert_eq!("t1", t.id);
            assert_eq!("content1", t.title);
            assert_eq!(memo.timestamp, t.timestamp);
        }
    }

    #[test]
    fn test_create_memo_fn_exist_topic() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Topic::create(&conn, "t1", "", 0).unwrap();
        }

        let memo = create_memo_fn(&data, "t1", "content2content2").unwrap();
        assert_ne!("", memo.id);
        assert_eq!("t1", memo.topic_id);
        assert_ne!(0, memo.timestamp);
        assert_eq!("content2content2", memo.content);

        {
            let conn = data.db.lock().unwrap();

            let memos = db::Memo::all_by_topic(&conn, "t1").unwrap();
            for m in memos {
                assert_ne!("", m.id);
                assert_eq!("t1", m.topic_id);
                assert_ne!(0, m.timestamp);
                assert_eq!("content2content2", m.content);
            }

            let topics = db::Topic::all(&conn).unwrap();
            let t = topics.iter().find(|t| t.id == "t1").unwrap();
            assert_eq!("t1", t.id);
            assert_eq!("content2co", t.title);
            assert_eq!(memo.timestamp, t.timestamp);
        }
    }

    #[test]
    fn test_delete_memo_fn_all() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
            db::Topic::create(&conn, "t1", "title1", 0).unwrap();
            db::TopicTag::create(&conn, "tag1", "t1").unwrap();
        }

        delete_memo_fn(&data, "t1", "m1").unwrap();

        {
            let conn = data.db.lock().unwrap();

            let memos = db::Memo::all_by_topic(&conn, "t1").unwrap();
            assert_eq!(0, memos.len());

            let topics = db::Topic::all(&conn).unwrap();
            let topic = topics.iter().filter(|t| t.id == "t1");
            assert_eq!(0, topic.count());

            let tags = db::TopicTag::all_by_topic(&conn, "t1").unwrap();
            assert_eq!(0, tags.len());
        }
    }

    #[test]
    fn test_delete_memo_fn_not_all() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
            db::Memo::create(&conn, "m2", "t1", 0, "content2").unwrap();
            db::Topic::create(&conn, "t1", "title1", 0).unwrap();
            db::TopicTag::create(&conn, "tag1", "t1").unwrap();
        }

        delete_memo_fn(&data, "t1", "m1").unwrap();

        {
            let conn = data.db.lock().unwrap();

            let memos = db::Memo::all_by_topic(&conn, "t1").unwrap();
            assert_eq!(1, memos.len());

            let topics = db::Topic::all(&conn).unwrap();
            let topic = topics.iter().filter(|t| t.id == "t1");
            assert_eq!(1, topic.count());

            let tags = db::TopicTag::all_by_topic(&conn, "t1").unwrap();
            assert_eq!(1, tags.len());
        }
    }

    #[test]
    fn test_get_memo_fn_latest() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
        }

        let memo = get_memo_fn(&data, "t1", Some("m1")).unwrap();
        assert_eq!("m1", memo.id);
        assert_eq!("t1", memo.topic_id);
        assert_eq!(0, memo.timestamp);
        assert!(memo.latest);
        assert_eq!("content1", memo.content);
    }

    #[test]
    fn test_get_memo_fn_no_latest() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
            db::Memo::create(&conn, "m2", "t1", 1, "content2").unwrap();
        }

        let memo = get_memo_fn(&data, "t1", Some("m1")).unwrap();
        assert_eq!("m1", memo.id);
        assert_eq!("t1", memo.topic_id);
        assert_eq!(0, memo.timestamp);
        assert!(!memo.latest);
        assert_eq!("content1", memo.content);
    }

    #[test]
    fn test_get_memo_fn_default() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
            db::Memo::create(&conn, "m2", "t1", 1, "content2").unwrap();
        }

        let memo = get_memo_fn(&data, "t1", None).unwrap();
        assert_eq!("m2", memo.id);
        assert_eq!("t1", memo.topic_id);
        assert_eq!(1, memo.timestamp);
        assert!(memo.latest);
        assert_eq!("content2", memo.content);
    }

    #[test]
    fn test_get_memo_fn_new() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
            db::Memo::create(&conn, "m2", "t1", 1, "content2").unwrap();
        }

        let memo = get_memo_fn(&data, "t2", None).unwrap();
        assert_eq!("", memo.id);
        assert_eq!("t2", memo.topic_id);
        assert_eq!(0, memo.timestamp);
        assert!(memo.latest);
        assert_eq!("", memo.content);
    }

    #[test]
    fn test_get_memo_all_fn() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
        }

        let memos = get_memo_all_fn(&data, "t1").unwrap();
        for m in memos {
            assert_eq!("m1", m.id);
            assert_eq!("t1", m.topic_id);
            assert_eq!(0, m.timestamp);
            assert!(m.latest);
            assert_eq!("content1", m.content);
        }
    }

    #[test]
    fn test_parse_title_short() {
        let t = parse_title("a");
        assert_eq!("a", &t);
    }

    #[test]
    fn test_parse_title_skip() {
        let t = parse_title("# a");
        assert_eq!("a", &t);

        let t = parse_title("## b ");
        assert_eq!("b", &t);
    }

    #[test]
    fn test_parse_title_cr() {
        let t = parse_title("a\nb");
        assert_eq!("a", &t);
    }

    #[test]
    fn test_parse_title_long() {
        let t = parse_title("01234567890");
        assert_eq!("0123456789", &t);
    }

    #[test]
    fn test_get_topics_fn_all() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Topic::create(&conn, "t1", "title1", 0).unwrap();
        }

        let topics = get_topics_fn(&data, "").unwrap();
        for t in topics {
            assert_eq!("t1", t.id);
            assert_eq!("title1", t.title);
            assert_eq!(0, t.timestamp);
        }
    }

    #[test]
    fn test_get_topics_fn_keyword() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::Topic::create(&conn, "t1", "title1", 0).unwrap();
            db::Memo::create(&conn, "m1", "t1", 0, "content1").unwrap();
        }

        let topics = get_topics_fn(&data, "tent").unwrap();
        for t in topics {
            assert_eq!("t1", t.id);
            assert_eq!("title1", t.title);
            assert_eq!(0, t.timestamp);
        }
    }

    #[test]
    fn test_add_memo_tag_fn() {
        let data = setup_appdate();

        add_memo_tag_fn(&data, "t1", "tag1").unwrap();

        {
            let conn = data.db.lock().unwrap();
            let tags = db::TopicTag::all_by_topic(&conn, "t1").unwrap();
            for t in tags {
                assert_eq!("tag1", t);
            }
        }
    }

    #[test]
    fn test_remove_memo_tag_fn() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::TopicTag::create(&conn, "tag1", "t1").unwrap();
        }

        remove_memo_tag_fn(&data, "t1", "tag1").unwrap();

        {
            let conn = data.db.lock().unwrap();
            let tags = db::TopicTag::all_by_topic(&conn, "t1").unwrap();
            assert_eq!(0, tags.len());
        }
    }

    #[test]
    fn test_get_memo_tag_fn() {
        let data = setup_appdate();
        {
            let conn = data.db.lock().unwrap();
            db::TopicTag::create(&conn, "tag1", "t1").unwrap();
        }

        let tags = get_memo_tag_fn(&data, "t1").unwrap();
        for tag in tags {
            assert_eq!("tag1", tag);
        }
    }

    fn setup_appdate() -> AppData {
        let conn = setup_connect();
        AppData {
            db: Mutex::new(conn),
        }
    }

    fn setup_connect() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_table_if_not_exists(&conn).unwrap();
        conn
    }
}
