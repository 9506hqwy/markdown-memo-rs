import { css, html } from "lit";
import { customElement } from "lit/decorators.js";
import type { Memo } from "./api";
import type { ContentAttr } from "./content-attr";
import type { ContentEditor } from "./content-editor";
import type { MemoCard } from "./memo-card";
import { MemoElement } from "./memo-element";
import type { NavigationMenu } from "./navigation-menu";
import type { OperationBar } from "./operation-bar";
import "./content-attr";
import "./content-editor";
import "./navigation-menu";
import "./operation-bar";
import "./search-bar";

/*
I dot not use <i> tag.

This solution does not work for me.
https://github.com/FortAwesome/Font-Awesome/issues/15316

Bacause this error is occured.
- Uncaught TypeError: MutationObserver.observe: Argument 1 is not an object.

So, I use raw node at `render()`.
*/

@customElement("memo-app")
export class MemoApp extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      div {
        width: 100vw;
        height: 100vh;
        color: #ffffff;
        background-color: #222222;
        display: flex;
        flex-direction: column;
      }

      header {
        height: 40px;
        display: flex;
        justify-content: flex-end;
      }

      search-bar {
        width: 100%;
      }

      main {
        height: calc(100% - 40px);
        display: flex;
      }

      article {
        width: 100%;
        display: flex;
        justify-content: flex-end;
      }

      content-editor {
        width: 100%;
        color: #000000;
        background-color: #ffffff;
      }
    `,
  ];

  override render() {
    return html`
      <div>
        <header>
          <search-bar></search-bar>
          <operation-bar></operation-bar>
        </header>
        <main>
          <navigation-menu></navigation-menu>
          <article>
            <content-editor></content-editor>
            <content-attr></content-attr>
          </article>
        </main>
      </div>
    `;
  }

  override firstUpdated() {
    const searchBar = this.renderRoot.querySelector("search-bar")!;
    const opBar = this.renderRoot.querySelector<OperationBar>("operation-bar")!;
    const navMenu =
      this.renderRoot.querySelector<NavigationMenu>("navigation-menu")!;
    const editor =
      this.renderRoot.querySelector<ContentEditor>("content-editor")!;
    const attr = this.renderRoot.querySelector<ContentAttr>("content-attr")!;

    searchBar.addEventListener("mm-keyword-change", (e) => {
      const ce = e as CustomEvent;
      navMenu.keyword = ce.detail.keyword;
    });

    opBar.addEventListener("mm-memo-save-request", () => {
      editor.save();
    });

    navMenu.addEventListener("mm-topic-change", (e) => {
      const ce = e as CustomEvent;
      const topic = ce.detail.topic as MemoCard;

      if (!editor.modified || window.confirm("Discard changes ?")) {
        opBar.setStatus(false);

        if (!topic.current) {
          editor.topicId = topic.cardId;
          editor.basecardId = undefined;
          attr.refresh(topic.cardId, undefined);
        } else {
          // Create new topic.
          const topicId = self.crypto.randomUUID();
          editor.topicId = topicId;
          editor.basecardId = undefined;
          attr.refresh(topicId, undefined);
        }
      } else {
        e.preventDefault();
      }
    });

    editor.addEventListener("mm-memo-created", (e) => {
      const ce = e as CustomEvent;
      const memo: Memo = ce.detail.memo;

      navMenu.refresh(editor.topicId);
      attr.refresh(editor.topicId, memo.id);
    });

    editor.addEventListener("mm-memo-status-changed", (e) => {
      const ce = e as CustomEvent;
      opBar.setStatus(ce.detail.modified);
    });

    attr.addEventListener("mm-memo-change", (e) => {
      const ce = e as CustomEvent;
      const memo = ce.detail.memo as MemoCard;

      if (!editor.modified || window.confirm("Discard changes ?")) {
        opBar.setStatus(false);
        editor.basecardId = memo.cardId;
      } else {
        e.preventDefault();
      }
    });

    attr.addEventListener("mm-memo-deleted", () => {
      editor.refresh();
    });

    this.addEventListener("mm-collapsed-changed", () => {
      // Do not use `getBoundingClientRect`.
      // Because element width not changed at this timing.
      const navWidth = navMenu.collapsed ? 50 : 250;
      const attrWidth = attr.collapsed ? 50 : 250;
      const sideWidth = navWidth + attrWidth;
      editor.setWidth(`calc(100vw - ${sideWidth}px)`);
    });
  }
}
