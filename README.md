# kg-ui-toolkit
Reusable components of the Rust toolkit for reuse in applications

## Functional goals

1. choice of front page templates - e.g. generic capability demo with links to info about specific capabilities
2. Generic object display with the following:
  - object defined by URI
  - discovered type of object used to access REST API to get type display rules (passing application name and any "parent" object type - which may be ignored or may trigger custom views)
  - display rules - choose properties to display ( default = all)
  - scalar (literal) properties - datatype specific widgets
  - object properties - display rule option as link with discovered label or nested object with type specific widget (recursive - see note about parent type above)
  - provenance trace unpacking - all objects may have provenance in our meta-model.
  - collection (parent container) navigation - find out more about the object from its container - if known.  (NB the object property to define the container will vary from type to type)
3. simple Map based search and display - with object display on click on map or tabular view of results
4. More functions TBD.

## Usage

The project is intended to be used as a git-submodule from within the UI project. It is also expected to be
used as a Cargo workspace member.

_This project also uses the OGC Building Blocks model to define how components interact with information architecture aspects_