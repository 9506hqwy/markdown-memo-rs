import { markdown } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import { EditorState, type Extension } from "@codemirror/state";
import { Task } from "@lit/task";
import { basicSetup, EditorView } from "codemirror";
import { css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import type { Memo } from "./api";
import { createMemo, getMemo } from "./api";
import { MemoElement } from "./memo-element";

@customElement("content-editor")
export class ContentEditor extends MemoElement {
  constructor() {
    super();

    this.domEventHandlers = EditorView.domEventHandlers({
      keydown: (event, _view) => {
        if (event.ctrlKey && event.key === "s") {
          event.preventDefault();
          this.save();
        }
      },
    });
  }

  static override styles = [
    MemoElement.styles,
    css`
      .md-editor {
        height: 100%;
        width: calc(100vw - 500px); /* hardcoded at main.ts */
      }

      .cm-editor {
        height: 100%;
      }

      .loading {
        display: flex;
        flex-direction: column;
      }
    `,
  ];

  // `undefined` means latest.
  @property()
  basecardId: string | undefined;

  @property()
  topicId: string = self.crypto.randomUUID();

  domEventHandlers: Extension;

  view: EditorView | undefined;

  modified: boolean = false;

  override render() {
    if (this.view) {
      this.view.destroy();
      this.view = undefined;
      this.modified = false;
    }

    return html`
      <div class="md-editor">
        ${this.loadTask.render({
          initial: () =>
            html`<div class="loading">
              ${this.renderLoading()}
              <div></div>
            </div>`,
          pending: () =>
            html`<div class="loading">
              ${this.renderLoading()}
              <div></div>
            </div>`,
          complete: (memo) => this.renderEditor(memo),
          error: (error) => html`<p>Error: ${error}</p>`,
        })}
      </div>
    `;
  }

  save() {
    if (this.modified) {
      if (this.view?.state.doc) {
        const content = this.view?.state.doc.toString();
        createMemo(this.topicId, content).then((m) => {
          this.dispatchMemoCreatedEvent(m);
        });
      }
    }

    this.updateModified(false);
  }

  setWidth(width: string) {
    const editor = this.renderRoot.querySelector<HTMLElement>(".md-editor")!;
    editor.style.width = width;
  }

  private loadTask = new Task(this, {
    task: async ([topicId, cardId]) => {
      return await getMemo(topicId!, cardId);
    },
    args: () => [this.topicId, this.basecardId],
  });

  private renderEditor(memo: Memo) {
    const extensions = [
      basicSetup,
      markdown({ codeLanguages: languages }),
      this.domEventHandlers,
      //EditorView.lineWrapping,
      EditorView.updateListener.of((e) => {
        if (e.docChanged) {
          this.updateModified(true);
        }
      }),
    ];

    if (!memo.latest) {
      extensions.push(EditorState.readOnly.of(true));
    }

    this.view = new EditorView({
      doc: memo.content,
      extensions: extensions,
      parent: this.renderRoot.querySelector("div.md-editor") as HTMLElement,
    });
  }

  private updateModified(modified: boolean) {
    const prevStatus = this.modified;
    this.modified = modified;

    if (prevStatus !== this.modified) {
      this.dispatchMemoStatusChangedEvent(this.modified);
    }
  }
}
