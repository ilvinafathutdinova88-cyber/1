import type { ButtonHTMLAttributes, ReactNode } from "react";
import { useStore } from "../store.ts";
import { translate as t } from "../i18n.ts";

/**
 * One screen on the wall: a black panel with a thin frame and a single-stroke title.
 * The body always has a positive size (see .screen-body in styles.css); canvases fill it.
 * A screen whose content names itself (the head) has no title: `lead` takes its place in the
 * header, and `label` names the screen for assistive tech.
 */
export function Screen(props: { title?: string; sub?: string; lead?: ReactNode; focus?: boolean; className?: string; tools?: ReactNode; children: ReactNode; label?: string }) {
  const { title, sub, lead, focus, className, tools, children, label } = props;
  const language = useStore((s) => s.language);
  return (
    <section className={`screen${focus ? " focus" : ""}${className ? " " + className : ""}`} aria-label={label ? t(label, language) : title ? t(title, language) : undefined}>
      {(title || lead || sub || tools) && (
        <header className="screen-head">
          {title && <h2 className="screen-title">{t(title, language)}</h2>}
          {lead}
          {sub && <span className="screen-sub">{t(sub, language)}</span>}
          {tools && <div className="screen-tools">{tools}</div>}
        </header>
      )}
      <div className="screen-body">{children}</div>
    </section>
  );
}

/** A framed caps button; `on` lights it and sets aria-pressed (a toggle) unless it's a plain action. */
export function Chip(props: ButtonHTMLAttributes<HTMLButtonElement> & { on?: boolean; toggle?: boolean; tone?: "slot0" | "slot1" | "slot2" | "slot3" | "amber" }) {
  const { on, toggle = true, tone, className, children, ...rest } = props;
  const language = useStore((s) => s.language);
  return (
    <button type="button" {...rest} className={`chip${on ? " on" : ""}${tone ? " " + tone : ""}${className ? " " + className : ""}`} aria-pressed={toggle ? !!on : undefined}>
      {typeof children === "string" ? t(children, language) : children}
    </button>
  );
}
