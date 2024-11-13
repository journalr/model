# model

A model for a journal written in rust.
A journal consists of at least one page. Pages have no fixed size and store elements in an r*-tree.

Elements are immutable as long as they are part of the journal. To modify an element, it must be extracted.
Once modified, elements can be reinserted into the journal.

This restriction is a safeguard against invalidation of the r*-tree, which happens when the envelope of an element
changes while it is stored in the r*-tree.
