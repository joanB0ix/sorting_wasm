## sorting_wasm

My first try using WASM+Rust to create a library for one of my small projects.

This library currently implements:
* Bubble sort.
* Insertion sort.
* Selection sort.
* Quick sort.
* Merge sort.
* Heap sort.

Each one of them return as a result all the steps as snapshots taken to achieve the sorting, coupled with the two items that were being swapped at each point.

Since snapshots take lot's of memory, therefore the library offers the option to not return them and only return the delta.