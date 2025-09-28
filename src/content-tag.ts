import { icon } from "@fortawesome/fontawesome-svg-core";
import { Task } from "@lit/task";
import { css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { addMemoTag, getMemoTag, removeMemoTag } from "./api";
import { MemoElement } from "./memo-element";

@customElement("content-tag")
export class ContentTag extends MemoElement {
  static override styles = [
    MemoElement.styles,
    css`
      p {
        margin: 2px;
        padding: 3px;
        display: inline-flex;
        flex-wrap: wrap;
        align-content: center;
        border-radius: 30px;
        background-color: #008800;
      }

      p:has(span.add) {
        background-color: #0000ff;
      }

      span {
        margin: 0px 5px;
      }

      span.add {
        min-width: 50px;
      }

      span:empty::before {
        content: "New Tag";
      }

      span:empty:focus::before {
        content: "";
      }

      svg {
        width: 14px;
        height: 14px;
      }

      svg.fa-square-minus,
      svg.fa-square-plus {
        cursor: pointer;
        color: #cccccc;
      }
    `,
  ];

  @property()
  topicId: string = "";

  @property()
  name: string = "";

  override render() {
    const content =
      this.name === ""
        ? html`<span
            contenteditable="true"
            class="add"
            @keydown=${this.enterTag}
          ></span> `
        : html`<span>${this.name}</span>`;

    const opIcon =
      this.name === ""
        ? icon({ prefix: "far", iconName: "square-plus" })
        : icon({ prefix: "far", iconName: "square-minus" });

    const clickOp = this.name === "" ? this.addTag : this.removeTag;

    return html`
      <p>${content} <span @click=${clickOp}>${opIcon.node[0]}</span></p>
    `;
  }

  private addTag(e: Event) {
    e.preventDefault();
    e.stopPropagation();

    const tag = this.renderRoot.querySelector("p:first-child")!;
    const content = tag.textContent.trim();

    if (this.topicId === "") {
      window.alert(`Save memo before adding tag. '${content}'`);
      return;
    }

    if (content.match(/\s/)) {
      window.alert(`Do not contains white space. '${content}'`);
      return;
    }

    if (content.length < 2) {
      window.alert(`Need two or more characters. '${content}'`);
      return;
    }

    this.dispatchTagAddRequestEvent(content);
  }

  private enterTag(e: KeyboardEvent) {
    if (e.type === "keydown" && e.key === "Enter") {
      this.addTag(e);
    }
  }

  private removeTag(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    if (window.confirm("Remove ?")) {
      const task = new Task(this, {
        task: async () => {
          await removeMemoTag(this.topicId, this.name);
          this.dispatchTagChangedEvent(this.name);
        },
      });
      task.run();
    }
  }
}

@customElement("content-tags")
export class ContentTags extends MemoElement {
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

  override render() {
    return this.loadTask.render({
      initial: () => html`<div>${this.renderLoading()}</div>`,
      pending: () => html`<div>${this.renderLoading()}</div>`,
      complete: (tags) => this.renderTags(tags),
      error: (error) => html`<p>Error: ${error}</p>`,
    });
  }

  override firstUpdated() {
    this.renderRoot.addEventListener("mm-tag-add-request", (e) => {
      const ce = e as CustomEvent;
      const tag: string = ce.detail.tag;

      for (const t of this.renderRoot.querySelectorAll<ContentTag>(
        "content-tag",
      )) {
        if (t.name === tag) {
          window.alert(`Already exists tag. '${tag}'`);
          return;
        }
      }

      if (window.confirm("Add ?")) {
        const task = new Task(this, {
          task: async () => {
            await addMemoTag(this.topicId, tag);
            this.loadTask.run();
            this.dispatchTagChangedEvent(tag);
          },
        });
        task.run();
      }
    });

    this.renderRoot.addEventListener("mm-tag-changed", () => {
      this.loadTask.run();
    });
  }

  private loadTask = new Task(this, {
    task: async ([topicId]) => {
      return await getMemoTag(topicId!);
    },
    args: () => [this.topicId],
  });

  private renderTags(tags: string[]) {
    const cards = [];
    for (const tag of tags) {
      cards.push(html`
        <content-tag topicId="${this.topicId}" name="${tag}"></memo-card>
      `);
    }

    cards.push(
      html`<content-tag topicId="${this.topicId}" name=""></content-tag>`,
    );
    return html`${cards}`;
  }
}
