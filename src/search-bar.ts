import { icon } from "@fortawesome/fontawesome-svg-core";
import { css, html } from "lit";
import { query } from "lit/decorators/query.js";
import { customElement } from "lit/decorators.js";
import { MemoElement } from "./memo-element";

@customElement("search-bar")
export class SearchBar extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      search {
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

      input {
        width: 95%;
        padding: 5px 15px;
        border-radius: 30px;
      }
    `,
  ];

  @query("input")
  keywordInput!: HTMLInputElement;

  override render() {
    const searchIcon = icon({ prefix: "fas", iconName: "magnifying-glass" });

    return html`
      <search>
        <div>
          ${searchIcon.node[0]}
          <input type="text" id="keyword" placeholder="keyword or #tag" />
        </div>
      </search>
    `;
  }

  override firstUpdated() {
    this.keywordInput.addEventListener("change", () => {
      this.dispatchKeywordChangeEvent(this.keywordInput.value);
    });
  }
}
