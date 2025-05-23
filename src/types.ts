export type VisualizerSettingInfo = {
  input: string;
  output: string;
  seed: number;
  turn: number;
  maxTurn: number;
};

export type VisualizerResult = {
  svgString: string;
  err: string;
  score: number;
  cmd: string;
  before_comment: string;
  after_comment: string;
};
