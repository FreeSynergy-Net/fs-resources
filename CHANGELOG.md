# Changelog

Version = tag; all crates of the workspace carry the same version.

## 0.0.2

### English

- **Window and server (basics, workflow `grundfassung`).** New crates: `fs-resources-server` (the contract `resources.capnp` draft 0.1 over a local socket through `fs-rpc` v0.0.2; answers `capabilities` with no capability ids and the contract version, and `supportReport`; everything else is `unimplemented`), `fs-resources-client` (connect with the version handshake, `Hello::fetch`, contract check: same main level, minor level of the server not lower), `fs-resources-simple` (the frontend `simple` as a process of its own: icon next to the title, goal, what works today, what comes; depends only on the client and fs-components v0.0.25), `fs-resources-bin` (the command `fs-resources`: starter, `serve`, `--version`, `--help`).
- **Starter.** `fs-resources` starts or replaces the server (handshake of `fs-rpc`), asks the frontend to check the contract (`--check`, exit code 3 = does not fit) and falls back to the standard frontend `simple` with a visible message; a missing or unfitting server is said in the window.
- **"What comes"** lists the next steps of the feature list (list and care of things, rooms and material, reserving with the reason for a refusal, availability strip, finishing a reservation, needs and offers, keyboard); your own time and the recast on ValueFlows with typed storage are named as planned, not scheduled.
- **Pins lifted so the tree holds one source per library:** `fs-objects` v0.0.1 to v0.0.6, `fs-interfaces` v0.0.1 to v0.0.27, new `fs-rpc` v0.0.2, `fs-types` v0.0.1 (through them); `.fs-lock-sources.toml` strict. Minimum Rust 1.89 for the workspace.
- **CI:** `check.yml` and `release.yml` call fs-release at `v0.0.10`.
- The real icon (`packaging/net.freesynergy.Resources.png`, `.desktop`, embedded in the window); `fs-resources` depends on the package `fs-resources-simple` (deb `depends`, rpm `requires`).
- Contract: draft (`resources.capnp`, not locked). The basics are released with this draft contract.
- Not done: the window shows no things or reservations (the core is not connected to the server), nothing can be changed in it, no lease (the server keeps running after the window closes until it is stopped or replaced).

### Deutsch

- **Fenster und Server (Grundfassung, Workflow `grundfassung`).** Neue Crates: `fs-resources-server` (der Vertrag `resources.capnp` Entwurf 0.1 über einen lokalen Socket mit `fs-rpc` v0.0.2; antwortet auf `capabilities` ohne Fähigkeits-Kennungen mit der Vertragsversion und auf `supportReport`; alles andere ist `unimplemented`), `fs-resources-client` (Verbinden mit Versions-Handschlag, `Hello::fetch`, Vertragsprüfung: gleiche Hauptstufe, Nebenstufe des Servers nicht niedriger), `fs-resources-simple` (das Frontend `simple` als eigener Prozess: Symbol neben dem Titel, Ziel, was heute geht, was kommt; hängt nur am Client und an fs-components v0.0.25), `fs-resources-bin` (der Befehl `fs-resources`: Starter, `serve`, `--version`, `--help`).
- **Starter.** `fs-resources` startet oder ersetzt den Server (Handschlag von `fs-rpc`), lässt das Frontend den Vertrag prüfen (`--check`, Rückgabecode 3 = passt nicht) und fällt mit sichtbarer Meldung auf das Standard-Frontend `simple` zurück; ein fehlender oder unpassender Server steht im Fenster.
- **„Was kommt“** nennt die nächsten Schritte der Feature-Liste (Liste und Pflege von Dingen, Räumen und Material, Reservieren mit Begründung bei Ablehnung, Verfügbarkeits-Streifen, Reservierung abschließen, Bedarf und Angebot, Tastatur); die eigene Zeit und der Neuschnitt nach ValueFlows mit typisierter Speicherung sind als geplant, nicht terminiert benannt.
- **Pins angehoben, damit der Baum je Bibliothek eine Quelle hat:** `fs-objects` v0.0.1 auf v0.0.6, `fs-interfaces` v0.0.1 auf v0.0.27, neu `fs-rpc` v0.0.2, `fs-types` v0.0.1 (durch diese); `.fs-lock-sources.toml` streng. Mindest-Rust 1.89 für den Arbeitsbereich.
- **CI:** `check.yml` und `release.yml` rufen fs-release bei `v0.0.10` auf.
- Das echte Symbol (`packaging/net.freesynergy.Resources.png`, `.desktop`, im Fenster eingebettet); `fs-resources` hängt vom Paket `fs-resources-simple` ab (deb `depends`, rpm `requires`).
- Vertrag: Entwurf (`resources.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf.
- Nicht getan: das Fenster zeigt keine Dinge oder Reservierungen (der Kern ist nicht am Server angeschlossen), nichts lässt sich darin ändern, keine Lease (der Server läuft nach dem Schließen des Fensters weiter, bis er beendet oder ersetzt wird).
