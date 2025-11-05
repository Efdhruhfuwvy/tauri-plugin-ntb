export enum WindowControl {
  Minimize = "Minimize",
  Maximize = "Maximize",
  Close = "Close",
}

export interface WindowControls {
  left: WindowControl[];
  right: WindowControl[];
}

export enum WindowControlImageType {
  SVG = "SVG",
  Text = "Text",
}

export type WindowControlImage = SVGWindowControlImage | TextWindowControlImage;

export interface SVGWindowControlImage {
  type: WindowControlImageType.SVG;
  svg: string;
}

export interface TextWindowControlImage {
  type: WindowControlImageType.Text;
  font: string;
  size: number | null;
  text: string;
}

export interface WindowControlImages {
  unmaximized: Record<WindowControl, WindowControlImage>;
  maximized: Record<WindowControl, WindowControlImage>;
}

export enum DragBehaviorType {
  Immediate = "Immediate",
  Threshold = "Threshold",
}

export type DragBehavior = ImmediateDragBehavior | ThresholdDragBehavior;

export interface ImmediateDragBehavior {
  type: DragBehaviorType.Immediate;
}

export interface ThresholdDragBehavior {
  type: DragBehaviorType.Threshold;
  threshold: number;
}
