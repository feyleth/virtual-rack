export enum Media {
  Audio = "Audio",
  Video = "Video",
  Midi = "Midi",
  Unknow = "Unknow",
  None = "None",
}
export enum NodeState {
  Creating = "Creating",
  Suspended = "Suspended",
  Idle = "Idle",
  Running = "Running",
  Error = "Error",
}
export enum NodeTypeDirection {
  In = "In",
  Out = "Out",
  Both = "Both",
  None = "None",
}
export interface Node {
  id: number;
  name: string;
  state: NodeState;
  media: Media;
  ports: Port[];
  nodeType: NodeTypeDirection;
}

export enum Direction {
  In = "In",
  Out = "Out",
}
export enum Format {
  Audio = "Audio",
  Video = "Video",
  Midi = "Midi",
  Unknow = "Unknow",
  None = "None",
}
export interface Port {
  id: number;
  name: string;
  direction: Direction;
  format: Format;
}

export interface Link {
  id: number,
  node_from: number,
  node_to: number,
  port_from: number,
  port_to: number,
  state: LinkState,
}

export enum LinkState {
  Error = "ErrorError",
  Unlinked = "Unlinked",
  Init = "Init",
  Negotiating = "NegotiatingNegotiating",
  Allocating = "AllocatingAllocating",
  Paused = "PausedPaused",
  Active = "ActiveActive",
}
