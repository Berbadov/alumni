import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

const stanzas = [
  `Let us go then, you and I,
When the evening is spread out against the sky
Like a patient etherized upon a table;
Let us go, through certain half-deserted streets,
The muttering retreats
Of restless nights in one-night cheap hotels
And sawdust restaurants with oyster-shells:
Streets that follow like a tedious argument
Of insidious intent
To lead you to an overwhelming question…
Oh, do not ask, "What is it?"
Let us go and make our visit.`,
  `In the room the women come and go
Talking of Michelangelo.`,
  `And indeed there will be time
To wonder, "Do I dare?" and, "Do I dare?"
Time to turn back and descend the stair,
With a bald spot in the middle of my hair—
(They will say: "How his hair is growing thin!")
My morning coat, my collar mounting firmly to the chin,
My necktie rich and modest, but asserted by a simple pin—
(They will say: "But how his arms and legs are thin!")
Do I dare
Disturb the universe?
In a minute there is time
For decisions and revisions which a minute will reverse.`,
  `I grow old…I grow old…
I shall wear the bottoms of my trousers rolled.

Shall I part my hair behind? Do I dare to eat a peach?
I shall wear white flannel trousers, and walk upon the beach.
I have heard the mermaids singing, each to each.

I do not think that they will sing to me.`,
]

export function About() {
  return (
    <div className="space-y-6 py-8">
      <h1 className="text-4xl font-bold tracking-tight">About</h1>
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">
            The Love Song of J. Alfred Prufrock
          </CardTitle>
          <CardDescription>T.S. Eliot, 1915 — excerpts</CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {stanzas.map((stanza) => (
            <p
              key={stanza.slice(0, 24)}
              className="whitespace-pre-line font-serif text-lg italic leading-relaxed text-card-foreground"
            >
              {stanza}
            </p>
          ))}
        </CardContent>
      </Card>
    </div>
  )
}
