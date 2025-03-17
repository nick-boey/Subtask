# User interface for Subtask

## Keybindings

- `h` - Go to parent task
- `j` - Go down one task
- `k` - Go up one task
- `l` - Go to child task
- `v` - Selection mode
- `ALT + j` - Move child up one level
- `ALT + k` - Move child down one level
- `TAB` - Demote subtask
- `SHIFT + TAB` - Promote subtask
- `f` - Toggle folding of task

## Relevant characters

```
Links: ╭╮╯╰─│
Tasks: ○ = not done
       ◉ = done
```

## Tasks

### Serial tasks

```
○ Parent task
╰─╮
  ◉ Serial subtask 1
  │ 
  ○ Serial subtask 2
╭─╯
○ Parent task
```

### Parallel tasks

```
○ Task
│
├─○ Task
│  
╰─○ Task

○ Task
```

## Actions

### Demote, serial tasks

Indenting with `TAB` results in a task being demoted such that it becomes a child of it's
former sibling.

```
○ Parent task
╰─╮
  ○ Subtask 
  │ 
->○ Subtask 
  │ 
  ○ Subtask 
╭─╯
○ Parent task
```

Results in:

```
○ Parent
╰─╮
  ○ Subtask
  ╰─╮ 
    ○ Subtask
  ╭─╯ 
  ○ Subtask
╭─╯
○ Parent
```

### Promote, serial tasks

Indenting with `TAB` results in a task being demoted such that it becomes a child of it's
former sibling.

```
○ Parent task
╰─╮
  ○ Subtask 
  │ 
  ○ Subtask 
  │ 
  ○ Subtask 
╭─╯
○ Parent task
```

Results in:

```
○ Parent
╰─╮
  ○ Subtask
╭─╯ 
○ Subtask
╰─╮ 
  ○ Subtask
╭─╯
○ Parent
```

### Demote, parallel tasks

Tasks become the child of the sibling above it, and the sibling adobts a default serial behaviour.

```
○ Task
│
├─○ Task
│
├─○ Task
│  
╰─○ Task

○ Task

Results in:

○ Task
│
├─○ Task
│ ╰─╮
│   ○ Task
│  
╰─○ Task
  
○ Task
```

### Promote, parallel tasks

This breaks a parallel task of it's chain, and the lower siblings
become a child of this task.

```
○ Task
│
├─○ Task
│
├─○ Task
│  
╰─○ Task

○ Task
```

Results in:

```
○ Task
│
╰─○ Task
 
○ Task
│  
╰─○ Task
  
○ Task
```
