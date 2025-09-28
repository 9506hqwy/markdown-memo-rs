import { Task } from "@lit/task";
import { css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import type { Memo } from "./api";
import { deleteMemo, getMemoHistory } from "./api";
import type { MemoCard } from "./memo-card";
import { MemoElement } from "./memo-element";
import "./memo-card";

@customElement("content-history")
export class ContentHistory extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      div {
        display: flex;
        flex-direction: column;
      }
    `,
  ];

  @property()
  topicId: string = "";

  currentHistoryId: string | undefined;

  override render() {
    return this.loadTask.render({
      initial: () => html`<div>${this.renderLoading()}</div>`,
      pending: () => html`<div>${this.renderLoading()}</div>`,
      complete: (tags) => this.renderMemos(tags),
      error: (error) => html`<p>Error: ${error}</p>`,
    });
  }

  override firstUpdated() {
    this.renderRoot.addEventListener("mm-memo-delete-request", (e) => {
      if (window.confirm("Delete ?")) {
        const ce = e as CustomEvent;
        const memo = ce.detail.memo as MemoCard;

        const task = new Task(this, {
          task: async () => {
            const remains = await deleteMemo(this.topicId, memo.cardId);
            this.currentHistoryId = undefined;
            this.loadTask.run();
            this.dispatchMemoDeletedEvent(remains);
          },
        });
        task.run();
      }
    });
  }

  refresh(topicId: string, cardId?: string) {
    this.currentHistoryId = cardId;

    if (this.topicId === topicId) {
      this.loadTask.run();
    } else {
      this.topicId = topicId;
    }
  }

  private clickMemo(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    const target = e.target as MemoCard;
    if (this.dispatchMemoChangeEvent(target)) {
      this.currentHistoryId = undefined;

      for (const member of this.renderRoot.querySelectorAll<MemoCard>(
        "memo-card",
      )) {
        if (target.cardId === member.cardId) {
          target.current = true;
          this.currentHistoryId = target.cardId;
        } else {
          member.current = false;
        }
      }
    }
  }

  private loadTask = new Task(this, {
    task: async ([topicId]) => {
      return await getMemoHistory(topicId!);
    },
    args: () => [this.topicId],
  });

  private renderMemos(memos: Memo[]) {
    const cards = [];
    for (const memo of memos) {
      if (this.currentHistoryId === undefined) {
        this.currentHistoryId = memo.id;
      }

      cards.push(html`
        <memo-card
          card-id="${memo.id}"
          card-title=""
          timestamp="${memo.timestamp}"
          ?current="${this.currentHistoryId === memo.id}"
          ?deletable="${true}"
          @click="${this.clickMemo}"
        ></memo-card>
      `);
    }

    return html`${cards}`;
  }
}
