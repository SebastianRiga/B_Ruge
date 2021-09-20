# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [B_Ruge]

### [0.2.8] - 19.09.2021

#### Added

* Added badge for web deployment preview to readme
* Added components and system to allow entities to drop items
* Added render order for entities and tiles
* Added components for the creation of items
* Added health potion item
* Added system to pick up and use items
* Added item and potion components
* Added potion usage system
* Added inventory dialog
* Added seeded random number generator

#### Changed

* Bumped version to 0.2.8 
* Game is now initialized with a seed based on the current time in nanoseconds
* Started to move plain text exception messages to a custom module for better maintainability
* Removed unnecessary domain alias from web preview deployment workflow yaml
* Separated web deployment workflow into preview for dev and production for main
* Reworked DialogInterface and DialogOptions to now incorporate a memory safe generic way to create callbacks and 
make them safely callable at any part of the game
* Reworked entity_factory spawning
* Modified deploy_web.yml to fire when pushed on dev and when a pull request is set for the main branch* Modified
deploy_web.yml to fire when pushed on dev and when a pull request is set for the main branch
* Update CHANGELOG.md
* Finished documentation for next release
* Moved remaining hardcoded colors to swatch 
* Updated manifest.json in the web templates to show the games correct name
* Updated inventory text for use and drop item
* Extended documentation

#### Fixes

* Fixed display issue of workflow badge deploy-web
* Corrected error in dialog display logic, causing dialogs without a text to display items of center 

### [0.2.7] - 11.09.2021

#### Added

* Added first UI with a message log, player health text and bar
* Added tooltips for entities on mouse hover
* Added indicator for mouse cursor position
* Added click-to-move functionality for the player
* Added a dialog interface to present generic dialogs at any point of the game

### [0.2.6] - 06.09.2021

#### Added

* Added chase mechanism for monsters
* Monsters are now seen as blocking entities and can't be walked over
* Finished damage system implementation

### [0.2.5] - 02.09.2021

#### Added

* Added missing header documentation for the systems module
* Added some monsters
* Added basic MonsterAI
* Added name component for entities
* Added factories for entities and base renderable

### [0.2.4] - 02.09.2021

#### Added

* Added field of view functionality to the map

### [0.2.3] - 01.09.2021

#### Added

* Updated map implementation to include rooms and intersections

## [0.2.2] - 31.08.2021

#### Added

* Added map to render on the screen
* Finished current documentation
* Project restructuring and cleanup

### [0.2.1] - 29.08.2021

#### Added

* Integrated basic entities, components and systems for rendering and movement.

### [0.1.0] - 29.08.2021

* Initial release which only shows the apps window in a simple hello world manner.

[B_Ruge]: https://github.com/SebastianRiga/B_Ruge
[0.2.8]: https://github.com/SebastianRiga/B_Ruge/Releases/Tag/v0.2.8
[0.2.7]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.7
[0.2.6]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.6
[0.2.5]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.5
[0.2.4]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.4
[0.2.3]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.3
[0.2.2]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.2
[0.2.1]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.2.1
[0.1.0]: https://github.com/SebastianRiga/B_Ruge/releases/tag/v0.1.0
