import { icon } from "@fortawesome/fontawesome-svg-core";
import { css, html } from "lit";
import { customElement } from "lit/decorators.js";
import { MemoElement } from "./memo-element";

@customElement("operation-bar")
export class OperationBar extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      section {
        width: 50px;
        padding: 5px;
      }

      div {
        display: flex;
        justify-content: center;
      }

      svg {
        width: 24px;
        padding: 0px 5px;
        color: #cccccc;
      }

      .modified {
        cursor: pointer;
        color: #ff6600;
      }
    `,
  ];

  override render() {
    const saveIcon = icon({ prefix: "far", iconName: "floppy-disk" });

    return html`
      <section>
        <div @click=${this.requstSave}>${saveIcon.node[0]}</div>
      </section>
    `;
  }

  setStatus(modified: boolean) {
    const icon = this.renderRoot.querySelector("svg")!;
    icon.classList.remove("modified");
    if (modified) {
      icon.classList.add("modified");
    }
  }

  private requstSave(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    const icon = this.renderRoot.querySelector("svg.fa-floppy-disk")!;
    if (!icon.classList.contains("modified")) {
      return;
    }

    this.dispatchMemoSaveRequestEvent();
  }
}
