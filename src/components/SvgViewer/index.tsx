import type { FC } from 'react';

type SvgViewerProps = {
  svgString: string;
  err: string;
  score: number;
  cmd: string;
};

const SvgViewer: FC<SvgViewerProps> = ({ svgString, err, score, cmd }) => {
  return (
    <>
      <div>
        score={score} {err && <span style={{ color: 'red' }}>({err})</span>}
      </div>
      <div>output: {cmd}</div>
      <div
        dangerouslySetInnerHTML={{
          __html: svgString,
        }}
      />
    </>
  );
};

export default SvgViewer;
