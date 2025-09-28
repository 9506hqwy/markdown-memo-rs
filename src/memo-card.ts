import { icon } from "@fortawesome/fontawesome-svg-core";
import { css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { MemoElement } from "./memo-element";

@customElement("memo-card")
export class MemoCard extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
    section {
      cursor: pointer;
      display: flex;
      align-items: center;
      padding: 5px;
    }

    section.selected {
      color: #FFFFAA;
      background-color: #555555;
    }

    section:hover {
      background-color: #666666;
    }

    div {
      overflow-x: hidden;
    }

    svg {
      width 24px;
      height: 24px;
      padding: 10px;
      color: #cccccc;
    }

    p {
      margin: 0px;
      padding: 0px;
      overflow-x: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
  `,
  ];

  @property({ attribute: "card-id" })
  cardId: string = "";

  @property({ attribute: "card-title" })
  cardTitle: string = "";

  @property({ type: Number })
  timestamp: number = 0;

  @property({ type: Boolean })
  current: boolean = false;

  @property({ type: Boolean })
  deletable: boolean = false;

  override render() {
    const secClass = this.current ? "selected" : "";
    const fileIcon = icon({ prefix: "fas", iconName: "file" });
    const deleteIcon = icon({ prefix: "far", iconName: "square-minus" });

    return html`
      <section class="${secClass}">
        ${fileIcon.node[0]}
        <div>
          <p>${this.cardTitle}</p>
          <p>${new Date(this.timestamp * 1000).toLocaleString()}</p>
        </div>
        ${
          this.deletable
            ? html`<p @click=${this.deleteHistory}>${deleteIcon.node[0]}</p>`
            : html``
        }
      </section>
    `;
  }

  private deleteHistory(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    this.dispatchMemoDeleteRequestEvent(this);
  }
}
