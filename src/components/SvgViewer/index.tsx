import type { FC } from 'react';

type SvgViewerProps = {
  svgString: string;
  err: string;
  score: number;
  cmd: string;
  beforeComment: string;
  afterComment: string;
};

const SvgViewer: FC<SvgViewerProps> = ({
  svgString,
  err,
  score,
  cmd,
  beforeComment,
  afterComment,
}) => {
  return (
    <>
      <div>
        score={score} {err && <span style={{ color: 'red' }}>({err})</span>}
      </div>
      <div style={{ display: 'flex', gap: '20px', alignItems: 'flex-start' }}>
        <div
          dangerouslySetInnerHTML={{
            __html: svgString,
          }}
        />
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div>cmd: {cmd}</div>
          <div>
            <p>before comment</p>
            <textarea
              className="beforeComment"
              id="beforeComment"
              rows={4}
              value={beforeComment}
            ></textarea>
          </div>
          <div>
            <p>after comment</p>
            <textarea
              className="afterComment"
              id="afterComment"
              rows={4}
              value={afterComment}
            ></textarea>
          </div>
        </div>
      </div>
    </>
  );
};

export default SvgViewer;
