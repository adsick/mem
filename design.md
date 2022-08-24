mem can be used as a cli application with custom commands syntax, it also will incorporate more interactive features and maybe a gui in the future.

## Usage

`mem note #rust you can use println!("{var}") syntax to print a var`

> created rust#4 note

`mem todo feed a cat`

> created todo#7
> `mem done todo#7`
> todo#7 done

`mem list todo`

`mem undo`

## Structure

### indexing

each document assigned a unique id, it is used to refer to documents internally.
documents could be accessed by their id, but that is not so user-friendly, instead we will 'classify' the document like so: note#4, todo#7, rust-note#2 etc. this literally means "the fourth note, the seventh todo and the second note which has rust topic.

### Lists

lists are the reason I've came back to this project, it allows you to manage lists of content that you want to watch not missing reading important articles.

### default actions and inference

mem is made for busy people and it can guess what you want to save your time.
for example it can detect a link and put it your read list.

it may even recognize the type of content or scalp web pages in the future to automatically index the content.

`mem https://youtu.be/1t1_a1BZ04o`

> added video "How to NOT Fail a Technical Interview" to your watchlist

## Commands

`> mem todo feed the cat #today`
`created a new todo#3 deadline: today`

## Configs

### Configurable stuff
