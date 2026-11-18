import type { CSSProperties } from 'react'
import { ArrowUpRight } from 'lucide-react'
import { Link } from 'react-router-dom'

interface Stanza {
  lines: string[]
  refrain?: number[]
}

const stanzas: Stanza[] = [
  {
    lines: [
      'Let us go then, you and I,',
      'When the evening is spread out against the sky',
      'Like a patient etherized upon a table;',
      'Let us go, through certain half-deserted streets,',
      'The muttering retreats',
      'Of restless nights in one-night cheap hotels',
      'And sawdust restaurants with oyster-shells:',
      'Streets that follow like a tedious argument',
      'Of insidious intent',
      'To lead you to an overwhelming question…',
      'Oh, do not ask, “What is it?”',
      'Let us go and make our visit.',
    ],
  },
  {
    lines: [
      'In the room the women come and go',
      'Talking of Michelangelo.',
    ],
    refrain: [0, 1],
  },
  {
    lines: [
      'And indeed there will be time',
      'To wonder, “Do I dare?” and, “Do I dare?”',
      'Time to turn back and descend the stair,',
      'With a bald spot in the middle of my hair—',
      '(They will say: “How his hair is growing thin!”)',
      'My morning coat, my collar mounting firmly to the chin,',
      'My necktie rich and modest, but asserted by a simple pin—',
      '(They will say: “But how his arms and legs are thin!”)',
      'Do I dare',
      'Disturb the universe?',
      'In a minute there is time',
      'For decisions and revisions which a minute will reverse.',
    ],
  },
  {
    lines: [
      'I grow old…I grow old…',
      'I shall wear the bottoms of my trousers rolled.',
      '',
      'Shall I part my hair behind? Do I dare to eat a peach?',
      'I shall wear white flannel trousers, and walk upon the beach.',
      'I have heard the mermaids singing, each to each.',
      '',
      'I do not think that they will sing to me.',
    ],
    refrain: [0],
  },
]

const evidence = [
  {
    no: '01',
    finding: 'The directory was checked against the registrar’s record on 18 Nov 2026.',
    source: 'SOURCE / REGISTRAR · 2026-11-18',
    status: 'verified',
  },
  {
    no: '02',
    finding: 'Graduation years come from the class list of 1 Sep 2026.',
    source: 'SOURCE / CLASS LIST · 2026-09-01',
    status: 'archived',
  },
  {
    no: '03',
    finding: 'The poem is printed as an excerpt, with the original punctuation retained.',
    source: 'SOURCE / EDITOR · 2026-11-18',
    status: 'archived',
  },
] as const

const halftoneStyle = { '--dot': 'var(--blush)' } as CSSProperties

export function About() {
  return (
    <div className="bg-white text-ink">
      <section className="surface-cellar py-24">
        <div className="mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <div className="flex items-center justify-between gap-6 border-b border-blush/30 pb-4">
            <span className="type-label">01 / ABOUT</span>
            <span className="type-label">EST. 2026</span>
          </div>

          <div className="mt-16 grid grid-cols-1 items-center gap-16 lg:grid-cols-12">
            <div className="reveal lg:col-span-5">
              <span
                aria-hidden="true"
                className="mb-8 block h-0.5 w-16 bg-blush"
              />
              <h1 className="type-display text-bone">
                An archive of the <em className="italic">unfinished</em>.
              </h1>
              <p className="type-lede mt-8 max-w-md text-bone/80">
                The directory keeps what each graduating year leaves behind.
              </p>
            </div>

            <div className="lg:col-span-7">
              <div className="relative aspect-[21/9] w-full overflow-hidden rounded-plate border border-blush/30 bg-cellar">
                <div
                  aria-hidden="true"
                  className="halftone absolute inset-x-0 top-1/4 h-1/2 opacity-70"
                  style={halftoneStyle}
                />
                <svg
                  aria-hidden="true"
                  className="absolute inset-0 h-full w-full"
                  viewBox="0 0 420 180"
                  fill="none"
                  preserveAspectRatio="xMidYMid meet"
                >
                  <path
                    d="M24 150H396"
                    stroke="var(--blush)"
                    strokeOpacity="0.3"
                  />
                  <path
                    d="M24 132H84V108H152V88H220V62H296V44H396"
                    stroke="var(--blush)"
                    strokeOpacity="0.55"
                  />
                  {[
                    [84, 108],
                    [152, 88],
                    [220, 62],
                    [296, 44],
                  ].map(([x, y]) => (
                    <rect
                      key={x}
                      x={x - 1.5}
                      y={y - 1.5}
                      width="3"
                      height="3"
                      fill="var(--blush)"
                    />
                  ))}
                </svg>
                <span className="type-label absolute left-4 top-4">REEL / 01</span>
                <span className="type-label absolute bottom-4 right-4">
                  ARCHIVE · 2026
                </span>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="bg-white py-24">
        <div className="relative mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <div
            aria-hidden="true"
            className="absolute inset-y-0 left-6 hidden w-px bg-line lg:block"
          >
            <span className="absolute left-1/2 top-24 h-[3px] w-[3px] -translate-x-1/2 bg-mulberry" />
            <span className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rotate-180 whitespace-nowrap type-label text-ink-muted [writing-mode:vertical-rl]">
              ALUMNI / DIRECTORY
            </span>
          </div>

          <div className="grid grid-cols-1 gap-16 lg:grid-cols-12">
            <div className="lg:col-span-5">
              <h2 className="type-h2">
                What the ledger <em className="italic text-mulberry">cannot</em> hold.
              </h2>
              <p className="mt-8 max-w-prose text-base leading-[1.6] text-ink">
                Each entry reduces a person to a degree, a graduation year, and a
                place. The reduction is useful. It is not the whole of the record.
              </p>
              <p className="mt-6 max-w-prose text-base leading-[1.6] text-ink">
                The poem below sits beside the data for the part that refuses to be
                counted: a voice deciding whether to speak.
              </p>
            </div>

            <div className="lg:col-span-7">
              <h3 className="type-h3 mb-8">
                The Love Song of J. Alfred Prufrock
              </h3>
              <blockquote className="border-t border-line pt-2">
                {stanzas.map((stanza, i) => (
                  <div
                    key={stanza.lines[0]}
                    className="grid grid-cols-[2rem_1fr] gap-4 border-t border-line py-6"
                  >
                    <span className="type-data text-ink-muted">
                      {String(i + 1).padStart(2, '0')}
                    </span>
                    <p
                      className={
                        i === 0
                          ? 'whitespace-pre-line type-quote not-italic'
                          : 'whitespace-pre-line font-display text-xl leading-[1.6]'
                      }
                    >
                      {stanza.lines.map((line, j) => (
                        <span
                          key={j}
                          className={stanza.refrain?.includes(j) ? 'italic' : undefined}
                        >
                          {j > 0 ? '\n' : ''}{line}
                        </span>
                      ))}
                    </p>
                  </div>
                ))}
                <cite className="type-label mt-4 block not-italic text-ink-muted">
                  T.S. Eliot, 1915 — excerpts
                </cite>
              </blockquote>
            </div>
          </div>

          <div className="mt-24">
            <h2 className="type-h2">Sources</h2>
            <div className="mt-8 border-t border-line">
              {evidence.map((item) => (
                <div
                  key={item.no}
                  className="grid grid-cols-[2rem_1fr] gap-x-4 gap-y-2 border-b border-line py-6 sm:grid-cols-[2rem_1fr_auto] sm:gap-x-6"
                >
                  <span className="type-data text-ink-muted">{item.no}</span>
                  <div>
                    <p className="type-ui text-ink">{item.finding}</p>
                    <p className="type-label mt-2 text-ink-muted">{item.source}</p>
                  </div>
                  <span className="type-label col-start-2 inline-flex items-center gap-2 text-ink-muted sm:col-start-auto sm:self-start">
                    {item.status === 'verified' ? (
                      <span aria-hidden="true" className="live-dot" />
                    ) : (
                      <span aria-hidden="true" className="status-dot" />
                    )}
                    {item.status}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </section>

      <section className="surface-cellar py-24">
        <h2 className="sr-only">Afterword</h2>
        <div className="mx-auto w-full max-w-[1280px] px-6 md:px-12">
          <span
            aria-hidden="true"
            className="mb-10 block h-px w-16 bg-blush"
          />
          <blockquote>
            <p className="type-quote max-w-3xl whitespace-pre-line text-bone">
              {`I have heard the mermaids singing, each to each.

I do not think that they will sing to me.`}
            </p>
            <cite className="type-label mt-8 block not-italic">
              T.S. Eliot, The Love Song of J. Alfred Prufrock, 1915
            </cite>
          </blockquote>
        </div>
      </section>

      <section className="border-t border-line bg-white py-10">
        <div className="mx-auto flex w-full max-w-[1280px] flex-wrap items-center justify-between gap-6 px-6 md:px-12">
          <span className="type-label text-ink-muted">
            ALUMNI / DIRECTORY · THE LIVING LEDGER
          </span>
          <Link to="/alumni" className="arrow-link type-ui text-mulberry">
            <span>Back to the directory</span>
            <span className="arrow-ring">
              <ArrowUpRight aria-hidden="true" className="size-5" strokeWidth={1.5} />
            </span>
          </Link>
        </div>
      </section>
    </div>
  )
}
