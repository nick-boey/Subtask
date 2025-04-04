# User interface for Subtask

## Todo list

1. Add and delete tasks
2. Navigate using depths
3. Folding tasks

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

### Serial and parallel tasks

```
○╮ Parent task
│╰─○ Serial subtask
│  ○ Serial subtask
○╮ Root task
│├─○ Parallel subtask
│╰─○╮ Parallel subtask
│   ╰─○ Serial subtask
│     ○ Serial subtask  
○  Root task 
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

# Logging

Log path is

```
C:\Users\<user>\AppData\Local\nick-boey\Subtask\data\Subtask.log
```