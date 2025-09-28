import { invoke } from "@tauri-apps/api/core";

export interface Memo {
  id: string;
  topicId: string;
  timestamp: number;
  latest: boolean;
  content: string;
}

export interface Topic {
  id: string;
  title: string;
  timestamp: number;
}

export async function addMemoTag(topicId: string, tag: string) {
  return await invoke("add_memo_tag", { topicId, tag });
}

export async function createMemo(
  topicId: string,
  content: string,
): Promise<Memo> {
  return await invoke("create_memo", { topicId, content });
}

export async function deleteMemo(topicId: string, id: string): Promise<number> {
  return await invoke("delete_memo", { topicId, id });
}

export async function getMemo(topicId: string, id?: string): Promise<Memo> {
  return await invoke("get_memo", { topicId, id });
}

export async function getMemoHistory(topicId: string): Promise<Memo[]> {
  return await invoke("get_memo_all", { topicId });
}

export async function getMemoTag(topicId: string): Promise<string[]> {
  return await invoke("get_memo_tag", { topicId });
}

export async function getTopics(keyword: string): Promise<Topic[]> {
  return await invoke("get_topics", { keyword });
}

export async function removeMemoTag(topicId: string, tag: string) {
  return await invoke("remove_memo_tag", { topicId, tag });
}

/*
// prototype.
const memos: Memo[] = [];
const tags: { [key: string]: string[] } = {};

export async function addMemoTag(topicId: string, tag: string) {
  console.log(`call addMemoTag with topicId:${topicId} tag:${tag}`);
  tags[topicId] ||= [];
  tags[topicId].push(tag);
}

export async function createMemo(
  topicId: string,
  content: string,
): Promise<Memo> {
  console.log(`call createMemo with topicId:${topicId}`);
  for (const m of memos) {
    if (m.topicId === topicId) {
      m.latest = false;
    }
  }

  const m = {
    id: self.crypto.randomUUID(),
    topicId: topicId,
    timestamp: Math.floor(Date.now() / 1000),
    latest: true,
    content: content,
  };

  memos.push(m);
  return m;
}

export async function deleteMemo(topicId: string, id: string): Promise<number> {
  console.log(`call deleteMemo with topicId:${topicId} id:${id}`);
  const idx = memos.findIndex((a) => a.topicId === topicId && a.id === id);
  memos.splice(idx, 1);

  const mm = memos.filter((a) => a.topicId === topicId);
  if (mm.length === 0) {
    delete tags[topicId];
  } else {
    mm.sort((a, b) => b.timestamp - a.timestamp);
    mm[0]!.latest = true;
  }

  return mm.length;
}

export async function getMemo(topicId: string, id?: string): Promise<Memo> {
  console.log(`call getMemo with topicId:${topicId} id:${id}`);
  const m = memos.find(
    (a) =>
      a.topicId === topicId && ((id === undefined && a.latest) || a.id === id),
  );
  return (
    m || {
      id: "",
      topicId: topicId,
      timestamp: 0,
      latest: true,
      content: "",
    }
  );
}

export async function getMemoHistory(topicId: string): Promise<Memo[]> {
  console.log(`call getMemoHistory with topicId:${topicId}`);
  const r: Memo[] = [];
  for (const m of memos) {
    if (m.topicId === topicId) {
      r.push(m);
    }
  }
  r.sort((a, b) => b.timestamp - a.timestamp);
  return r;
}

export async function getMemoTag(topicId: string): Promise<string[]> {
  console.log(`call getMemoTag with topicId:${topicId}`);
  return tags[topicId] || [];
}

export async function getTopics(keyword: string): Promise<Topic[]> {
  console.log(`call getTopics with keyword:${keyword}`);
  const topics: Topic[] = [];
  for (const m of memos) {
    const t = topics.find((a) => a.id === m.topicId);
    if (t) {
      if (t.timestamp < m.timestamp) {
        t.timestamp = m.timestamp;
      }
    } else {
      topics.push({
        id: m.topicId,
        title: `t ${m.topicId}`.slice(0, 10),
        timestamp: m.timestamp,
      });
    }
  }
  topics.sort((a, b) => b.timestamp - a.timestamp);
  return topics;
}

export async function removeMemoTag(topicId: string, tag: string) {
  console.log(`call removeMemoTag with topicId:${topicId} tag:${tag}`);
  tags[topicId] ||= [];
  const idx = tags[topicId].indexOf(tag);
  tags[topicId].splice(idx, 1);
}
*/
