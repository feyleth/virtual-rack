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
export interface Node {
  id: number;
  name: string;
  state: NodeState;
  media: Media;
  ports: Port[];
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
