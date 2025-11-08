import { invoke } from "@tauri-apps/api/core";
import {
  DragBehavior,
  DragBehaviorType,
  ThresholdDragBehavior,
  WindowControl,
  WindowControlImage,
} from "./models";

export async function getWindowControlImages(): Promise<
  {
    unmaximized: Record<WindowControl, WindowControlImage | null>;
    maximized: Record<WindowControl, WindowControlImage | null>;
  }
> {
  return {
    unmaximized: {
      [WindowControl.Minimize]: await invoke(
        "plugin:ntb|get_window_control_image",
        {
          control: WindowControl.Minimize,
          maximized: false,
        },
      ),
      [WindowControl.Maximize]: await invoke(
        "plugin:ntb|get_window_control_image",
        {
          control: WindowControl.Maximize,
          maximized: false,
        },
      ),
      [WindowControl.Close]: await invoke(
        "plugin:ntb|get_window_control_image",
        {
          control: WindowControl.Close,
          maximized: false,
        },
      ),
    },
    maximized: {
      [WindowControl.Minimize]: await invoke(
        "plugin:ntb|get_window_control_image",
        {
          control: WindowControl.Minimize,
          maximized: true,
        },
      ),
      [WindowControl.Maximize]: await invoke(
        "plugin:ntb|get_window_control_image",
        {
          control: WindowControl.Maximize,
          maximized: true,
        },
      ),
      [WindowControl.Close]: await invoke(
        "plugin:ntb|get_window_control_image",
        {
          control: WindowControl.Close,
          maximized: true,
        },
      ),
    },
  };
}

export async function titleBarMouseDown(event: MouseEvent) {
  if (event.button === 0) {
    const dragBehavior: DragBehavior = await invoke(
      "plugin:ntb|get_drag_behavior",
    );
    if (dragBehavior.type === DragBehaviorType.Immediate) {
      let element = event.target as Element;
      while (element !== event.currentTarget) {
        if (
          element.tagName === "BUTTON" ||
          element.tagName === "INPUT" ||
          element.tagName === "TEXTAREA"
        ) {
          return;
        }
        element = element.parentNode as Element;
      }
      invoke("plugin:ntb|drag");
    } else if (dragBehavior.type === DragBehaviorType.Threshold) {
      async function mouseMove(event1: MouseEvent) {
        if (
          Math.max(
            Math.abs(event1.x - event.x),
            Math.abs(event1.y - event.y),
          ) >= (dragBehavior as ThresholdDragBehavior).threshold
        ) {
          removeEventListener("mousemove", mouseMove);
          removeEventListener("mouseup", mouseUp);
          addEventListener("click", (event) => event.stopPropagation(), {
            capture: true,
            once: true,
          });
          await invoke("plugin:ntb|move_by", {
            x: event1.x - event.x,
            y: event1.y - event.y,
          });
          invoke("plugin:ntb|drag");
        }
      }

      function mouseUp() {
        removeEventListener("mousemove", mouseMove);
        removeEventListener("mouseup", mouseUp);
      }

      addEventListener("mousemove", mouseMove);
      addEventListener("mouseup", mouseUp);
    }
  }
}

const styleSheet = new CSSStyleSheet();
document.adoptedStyleSheets.push(styleSheet);

async function updateCSS() {
  const style = getComputedStyle(document.body);
  const light = /\blight\b/.test(style.colorScheme);
  const dark = /\bdark\b/.test(style.colorScheme);
  styleSheet.replaceSync(
    await invoke("plugin:ntb|get_title_bar_css", {
      dark: light === dark
        ? matchMedia("(prefers-color-scheme: dark)").matches
        : dark,
    }),
  );
  for (const rule of styleSheet.cssRules) {
    if (rule instanceof CSSStyleRule) {
      rule.selectorText = `.title-bar :is(${
        rule.selectorText
          .replace(/windowcontrols/g, ".window-controls")
          .replace(/image/g, "svg")
      })`;
    }
  }
}

matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", updateCSS);
updateCSS();

let documentHovered = true;
let snapOverlay = false;
let updateSnapOverlayTimer: number | undefined;

function updateSnapOverlay() {
  if (maximizeButtonsHovered.some((hovered) => hovered)) {
    if (!snapOverlay) {
      snapOverlay = true;
      invoke("plugin:ntb|show_snap_overlay");
    }
  } else if (documentHovered) {
    if (snapOverlay) {
      snapOverlay = false;
      invoke("plugin:ntb|show_snap_overlay");
    }
  }
}

document.addEventListener("mouseenter", () => {
  documentHovered = true;
  clearTimeout(updateSnapOverlayTimer);
  updateSnapOverlayTimer = setTimeout(updateSnapOverlay, 500);
});
document.addEventListener("mouseleave", () => {
  documentHovered = false;
  clearTimeout(updateSnapOverlayTimer);
  updateSnapOverlayTimer = setTimeout(updateSnapOverlay, 500);
});

const maximizeButtonsHovered: boolean[] = [];

export function addMaximizeButton(button: Element) {
  let index = maximizeButtonsHovered.length;
  maximizeButtonsHovered[index] = false;
  button.addEventListener("mouseenter", () => {
    maximizeButtonsHovered[index] = true;
    clearTimeout(updateSnapOverlayTimer);
    updateSnapOverlayTimer = setTimeout(updateSnapOverlay, 500);
  });
  button.addEventListener("mouseleave", () => {
    maximizeButtonsHovered[index] = false;
    clearTimeout(updateSnapOverlayTimer);
    updateSnapOverlayTimer = setTimeout(updateSnapOverlay, 500);
  });
}
