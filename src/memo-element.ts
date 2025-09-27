import { icon, library } from "@fortawesome/fontawesome-svg-core";
import { far } from "@fortawesome/free-regular-svg-icons";
import { fas } from "@fortawesome/free-solid-svg-icons";
import { type CSSResultGroup, css, html, LitElement } from "lit";
import type { Memo } from "./api";
import type { MemoCard } from "./memo-card";

export class MemoElement extends LitElement {
  constructor() {
    super();
    library.add(far, fas);
  }

  static override styles: CSSResultGroup = [
    css`
      * {
        margin: 0;
        padding: 0;
      }

      svg.fa-spinner {
        width: 48px;
        height: 48px;
        align-self: center;
        animation: rotation 2s linear infinite;
      }

      @keyframes rotation {
        from {
          transform: rotate(0deg);
        }
        to {
          transform: rotate(360deg);
        }
      }
    `,
  ];

  protected dispatchCollapsedChangedEvent(): boolean {
    return this.dispatchCustomEvent("mm-collapsed-changed");
  }

  protected dispatchKeywordChangeEvent(keyword: string): boolean {
    return this.dispatchCustomEvent("mm-keyword-change", { keyword: keyword });
  }

  protected dispatchMemoChangeEvent(memo: MemoCard) {
    return this.dispatchCustomEvent("mm-memo-change", { memo: memo });
  }

  protected dispatchMemoCreatedEvent(memo: Memo) {
    return this.dispatchCustomEvent("mm-memo-created", { memo: memo });
  }

  protected dispatchMemoDeleteRequestEvent(memo: MemoCard) {
    return this.dispatchCustomEvent("mm-memo-delete-request", { memo: memo });
  }

  protected dispatchMemoSaveRequestEvent(): boolean {
    return this.dispatchCustomEvent("mm-memo-save-request");
  }

  protected dispatchMemoStatusChangedEvent(modified: boolean) {
    return this.dispatchCustomEvent("mm-memo-status-changed", {
      modified: modified,
    });
  }

  protected dispatchTagChangedEvent(tag: string) {
    return this.dispatchCustomEvent("mm-tag-changed", { tag: tag });
  }

  protected dispatchTopicChangeEvent(topic: MemoCard): boolean {
    return this.dispatchCustomEvent("mm-topic-change", { topic: topic });
  }

  protected renderLoading() {
    const spinner = icon({ prefix: "fas", iconName: "spinner" });
    return html`${spinner.node[0]}`;
  }

  private dispatchCustomEvent<T>(name: string, detail?: T): boolean {
    const event = new CustomEvent(name, {
      bubbles: true,
      cancelable: true,
      composed: true,
      detail: detail,
    });
    return this.dispatchEvent(event);
  }
}
