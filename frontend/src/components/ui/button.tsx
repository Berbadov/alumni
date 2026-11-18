import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"
import { cn } from "cn"
import { Slot } from "radix-ui"

const buttonVariants = cva(
  "inline-flex shrink-0 items-center justify-center gap-2 rounded-pill type-ui whitespace-nowrap transition-[background-color,border-color,color,transform] duration-[160ms] outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ink disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-oxblood active:scale-[0.98]",
        destructive: "bg-destructive text-white hover:bg-destructive/90 active:scale-[0.98]",
        outline:
          "border border-line-strong bg-transparent text-ink hover:border-ink hover:bg-blush",
        secondary: "bg-hay text-ink hover:bg-sage",
        ghost: "text-ink hover:bg-bone",
        link: "h-auto rounded-none p-0 text-mulberry underline underline-offset-4 hover:decoration-rose",
      },
      size: {
        default: "h-11 px-5",
        xs: "h-7 gap-1 px-3 text-xs",
        sm: "h-9 px-4",
        lg: "h-12 px-7 text-base",
        icon: "size-11",
        "icon-xs": "size-7",
        "icon-sm": "size-9",
        "icon-lg": "size-12",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean
  }) {
  const Comp = asChild ? Slot.Root : "button"

  return (
    <Comp
      data-slot="button"
      data-variant={variant}
      data-size={size}
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  )
}

export { Button, buttonVariants }
