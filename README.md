# FreeSynergy Resources — know what we have and need

**Two people cannot book the same room by accident, and nobody else can take your time.**

FreeSynergy Resources keeps what a group has and what it needs: things, rooms, material in stock, and people's time. You can reserve something for a stretch of time. You can post a need, and someone can post an offer.

What is different here: your own capacity, meaning your time, belongs to you. Only you decide what is shown and allocated, and what a group sees is a share you give. A reservation is checked in one step, so two people cannot overbook together. Material that is used up is counted as used up. Things and people's time are one program on purpose, because they share planning.

## What it can do today

Latest tag: `v0.0.1` (the core only; the tag of 0.0.2 is set at release). The version in this tree is `0.0.2` (program with a window, server and frontend).

- **A window and a server (new in 0.0.2).** `fs-resources` makes sure the server runs and opens the frontend `simple`: the name, the icon, what works today and what comes. Nothing in the window manages things yet; everything that does not work yet says "coming soon", open points say "planned, not scheduled", and a missing server or one that does not fit is said in the window, never a grey window. Try `fs-resources --version`, `fs-resources --help`.

- Things, rooms and material, with stock that is worked out from what has been finished.
- Reservations that cannot overbook; finishing a reservation frees the thing or uses up the material.
- Free time of a thing over a range.
- Your own capacity, allocations and shares to groups, kept in a store of its own.
- The load of a group, using the Groups interface.
- Needs and offers; accepting an offer makes the reservation.
- Tests: 115, with real stores. Checked by an independent reviewer over four rounds.

## What is not there yet

- The window shows no things, rooms or material and nothing can be reserved in it: the core is not connected to the server. The server answers only what it offers (nothing yet) and the contract version, and refuses your capacity calls until it knows who is asking. It keeps running after the window is closed until it is stopped or replaced (no lease).
- The recast on the ValueFlows model (kind of thing with a typed unit, hand-over, commitment, catalogue as a filter) together with typed storage: planned, not scheduled. A marketplace would later be a facet on offers and needs, not a program of its own.
- No costs, no MSPDI (project plan file) support, no skills.
- No warehouses, batches or valuation; this is availability, not an ERP.
- Contract: draft (`resources.capnp`, not locked). The basics are released with this draft contract. No standard for a resource model was found, so the interface is a draft marked provisional.

## Part of FreeSynergy

Resources belongs to the project-management and democracy suites. FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); the reading version is at <https://freesynergy.github.io/fs-documentation/>. Start with the [manifesto](https://freesynergy.github.io/fs-documentation/manifesto/) if you want to know why we do this.


## For developers

- Workspace crates: `fs-resources-core` (core, no GUI), `fs-resources-server` (the contract over a local socket), `fs-resources-client` (the contract as a client), `fs-resources-simple` (the frontend `simple`, a process of its own, depends only on the client and fs-components), `fs-resources-bin` (the command `fs-resources`: starter and `serve`).
- Engine and frontend are separate processes (ADR 0021, 0094 draft); the manifest of the frontend is `packaging/frontends/simple.metainfo.xml`.
- The contract is `resources.capnp` in [fs-interfaces](https://github.com/FreeSynergy/fs-interfaces) (draft, provisional). Resources owns it.
- Build and test: `cargo build --workspace` and `cargo test --workspace` (the window crates need the system libraries of Blinc; the tests do not open a window).
- Design and decisions: [DESIGN.md](DESIGN.md).
- Licence: MIT OR Apache-2.0.

## Auf Deutsch

**Zwei Menschen buchen nicht aus Versehen denselben Raum, und niemand sonst kann über deine Zeit verfügen.** FreeSynergy Resources hält fest, was eine Gruppe hat und braucht: Dinge, Räume, Material im Bestand und die Zeit von Menschen. Man kann etwas für eine Zeit reservieren, einen Bedarf einstellen, und jemand kann ein Angebot machen.

**Was es heute kann.** Letzter Tag `v0.0.1` (nur der Kern). Der Stand in diesem Baum ist `0.0.2` (Programm mit Fenster, Server und Frontend). Der Kern (Dinge, Räume, Material, Reservierungen ohne Überbuchung, freie Zeit, Bedarf und Angebot, die eigene Kapazität) liegt als Bibliothek vor. Neu ist ein Fenster mit Server: `fs-resources` startet den Server und das Frontend `simple` mit Name, Symbol, „Was es heute kann“ und „Was kommt“. Was noch nicht geht, sagt „kommt noch“, offene Punkte sagen „Geplant, nicht terminiert“; fehlt der Server oder passt er nicht, steht das im Fenster. Im Fenster lässt sich noch nichts verwalten, der Kern ist noch nicht daran angeschlossen.

**Was noch nicht da ist.** Liste und Pflege von Dingen, Räumen und Material, Reservieren mit Überbuchungs-Meldung, Verfügbarkeits-Streifen, Bedarf und Angebot im Fenster (kommt noch). Die eigene Zeit im Fenster und der Neuschnitt nach ValueFlows zusammen mit typisierter Speicherung sind geplant, nicht terminiert; ein Marktplatz wäre später eine Facette auf Angebot und Bedarf, kein eigenes Programm. Der Server läuft nach dem Schließen des Fensters weiter (keine Lease).

**Für Entwickler.** Frontend und Engine sind getrennte Prozesse; das Frontend hängt nur am Client und an fs-components. Bauen: `cargo build --workspace`, `cargo test --workspace`. Vertrag: Entwurf (`resources.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf. Lizenz: MIT OR Apache-2.0.
