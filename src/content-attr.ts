import { icon } from "@fortawesome/fontawesome-svg-core";
import { css, html } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import type { ContentHistory } from "./content-history";
import type { ContentTags } from "./content-tag";
import { MemoElement } from "./memo-element";
import "./content-history";
import "./content-tag";

@customElement("content-attr")
export class ContentAttr extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      aside {
        width: 250px; /* hardcoded at main.ts */
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
      }

      aside.collapsed {
        width: 50px; /* hardcoded at main.ts */
      }

      article {
        height: 100%;
        overflow-y: auto;
        flex-direction: column;
      }

      hr {
        border-style: dotted;
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
  topicId: string = "";

  @state()
  collapsed: boolean = false;

  override render() {
    const asideClass = this.collapsed ? "collapsed" : "";
    const articleDisplay = this.collapsed ? "none" : "flex";
    const expander = this.collapsed
      ? icon({ prefix: "fas", iconName: "up-right-and-down-left-from-center" })
      : icon({ prefix: "fas", iconName: "down-left-and-up-right-to-center" });

    return html`
      <aside class="${asideClass}">
        <article style="display: ${articleDisplay}">
          <content-tags topicId="${this.topicId}"></content-tags>
          <hr />
          <content-history topicId="${this.topicId}"></content-history>
        </article>
        <div @click="${this.toggle}">${expander.node[0]}</div>
      </aside>
    `;
  }

  refresh(topicId: string, cardId?: string) {
    const tags = this.renderRoot.querySelector<ContentTags>("content-tags")!;
    tags.topicId = topicId;

    const history =
      this.renderRoot.querySelector<ContentHistory>("content-history")!;
    history.refresh(topicId, cardId);
  }

  private toggle(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    this.collapsed = !this.collapsed;
    this.dispatchCollapsedChangedEvent();
  }
}
