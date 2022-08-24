basic operation:
- [ ] storing documents in local disk storage
- [ ] syncing with the server via internet connection
- [ ] collaborative features

documents can contain:
- [ ] text
- [ ] #tags
- [ ] links to other documents

documents also contain some meta information such as creation/edited date and other context.

meta:
    - creation date
    - last edit date
    - last viewed
    - times viewed
    - type specific date (e.g. answered for QA or done for todo)
    - comment


document types:
- [ ] general (note)
    - title
    - topic
    - content
    - tags
    - category(?)
- [ ] QA
  - question
  - answer (optional)
- [ ] todo
    - title
    - priority (default)
    - description
    - requirements
    - outcome (optional)
    - success criteria (optional)
    - deadline (optional)
    - status (default)
- [ ] card (e.g. for language learning)
  - A side
  - B side
  - note (optional)


presentation:
- [ ] colors
- [ ] pretty printing
- [ ] paging

every document has it's numerical id associated with it's type,
so you can link to 12th note simply by typing note#12 (syntax may change)

access:
- [ ] get by #id
- [ ] get by type#id
- [ ] get by topic-type#id
- [ ] find
  - [ ] by title
  - [ ] by content
- [ ] filter
  - [ ] by topic
  - [ ] by tags
  - [ ] boolean (queries based on logical expressions)
- [ ] recent changes (VecDeque<id>)


config ideas:
- [ ] save on exit, save on edit
- [ ] return default action #idk what it means but it was here.
- [ ] custom command shortcuts
- [ ] always show available commands list hint
- [ ] select or direct input command
