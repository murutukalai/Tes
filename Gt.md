Yes, you can delete a local Git branch automatically if it has been deleted from the remote repository. Here are a few ways to do it:

### 1. Using `git fetch --prune`
This command removes local tracking branches that no longer exist on the remote:
```sh
git fetch --prune
```

### 2. Using `git fetch -p` (shorter version)
```sh
git fetch -p
```
This will remove remote-tracking branches (i.e., `origin/branch-name`), but not local branches.

### 3. Delete local branches that are gone from remote
After pruning, you can remove local branches that no longer have a remote counterpart:
```sh
git branch -vv | awk '/: gone]/{print $1}' | xargs git branch -d
```
- `git branch -vv` lists all branches with their tracking info.
- `awk '/: gone]/{print $1}'` filters branches whose remote is gone.
- `xargs git branch -d` deletes them.

### 4. Force delete (if branches are not fully merged)
If you want to force delete the local branches that were deleted remotely, use:
```sh
git branch -vv | awk '/: gone]/{print $1}' | xargs git branch -D
```

Would you like to automate this in a script?
