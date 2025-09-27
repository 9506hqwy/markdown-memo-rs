import { icon } from "@fortawesome/fontawesome-svg-core";
import { Task } from "@lit/task";
import { css, html } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import type { Topic } from "./api";
import { getTopics } from "./api";
import type { MemoCard } from "./memo-card";
import { MemoElement } from "./memo-element";
import "./memo-card";

@customElement("navigation-menu")
export class NavigationMenu extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      nav {
        width: 200px; /* hardcoded at main.ts */
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
      }

      nav.collapsed {
        width: 50px; /* hardcoded at main.ts */
      }

      article {
        height: 100%;
        overflow-y: auto;
        flex-direction: column;
      }

      div {
        height: 50px;
        cursor: pointer;
        display: flex;
        justify-content: center;
      }

      div:hover {
        background-color: #888888;
      }

      svg {
        width: 24px;
        height: 24px;
        padding: 5px;
        rotate: 45deg;
        color: #cccccc;
      }
    `,
  ];

  @property()
  keyword: string = "";

  @state()
  collapsed: boolean = false;

  currentTopicId: string | undefined;

  override render() {
    const navClass = this.collapsed ? "collapsed" : "";
    const articleDisplay = this.collapsed ? "none" : "flex";
    const expander = this.collapsed
      ? icon({ prefix: "fas", iconName: "up-right-and-down-left-from-center" })
      : icon({ prefix: "fas", iconName: "down-left-and-up-right-to-center" });

    return html`
      <nav class="${navClass}">
        <article style="display: ${articleDisplay}">
          ${this.loadTask.render({
            initial: () => this.renderLoading(),
            pending: () => this.renderLoading(),
            complete: (topics) => this.renderTopics(topics),
            error: (error) => html`<p>Error: ${error}</p>`,
          })}
        </article>
        <div @click="${this.toggle}">${expander.node[0]}</div>
      </nav>
    `;
  }

  refresh(topicId: string) {
    this.currentTopicId = topicId;
    this.loadTask.run();
  }

  private clickTopic(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    const target = e.target as MemoCard;
    if (this.dispatchTopicChangeEvent(target)) {
      this.currentTopicId = undefined;

      for (const member of this.renderRoot.querySelectorAll<MemoCard>(
        "memo-card",
      )) {
        if (target.cardId === member.cardId) {
          target.current = !target.current;
          if (target.current) {
            this.currentTopicId = target.cardId;
          }
        } else {
          member.current = false;
        }
      }
    }
  }

  private loadTask = new Task(this, {
    task: async ([keyword]) => {
      const topics = await getTopics(keyword);
      return topics;
    },
    args: () => [this.keyword],
  });

  private renderTopics(topics: Topic[]) {
    const cards = [];
    for (const topic of topics) {
      cards.push(html`
        <memo-card
          card-id="${topic.id}"
          card-title="${topic.title}"
          timestamp="${topic.timestamp}"
          ?current="${this.currentTopicId === topic.id}"
          @click="${this.clickTopic}"
        ></memo-card>
      `);
    }

    return html`${cards}`;
  }

  private toggle(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    this.collapsed = !this.collapsed;
    this.dispatchCollapsedChangedEvent();
  }
}
