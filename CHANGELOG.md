# Changelog

## 0.1.5

- Update to Horizon Nightly 2026-11-05 (`66f3981`).
- Adapt first-person geometry to the hand/item shader ID contract.
- Preserve current hand-pose hooks, shield sizing and held-item resolution.
- Validate the new pin with a full native build and 12 fresh graphical checks.

## 0.1.4

- Simplify the project page around features, installation and compatibility.
- Remove personal maintainer branding from the project description and documentation.

## 0.1.3

- Extract the native port into a portable standalone source repository.
- Run engine tests using embedded resources and checked-in numerical fixtures.
- Package the native bundle with the pinned Horizon validator from an explicit source checkout.
- Retain the behavior of the locally tested 0.1.2 runtime.

## 0.1.2 (local runtime baseline)

- Connect consumption and mining start-loop-end controllers.
- Track successful item/block/entity uses and creative instant breaks.
- Keep live outgoing animation poses during interruptions.
- Respect declared loops, continuous expression clocks and timeline hand visibility.
- Keep camera/body animation shared across single-arm actions.
