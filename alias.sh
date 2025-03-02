# Find and remove the folders or files Eg: rmt target it remove all the targets inside your current folder
rmt() {
    find ./ -depth -name "$1" -exec rm -rf {} +
}
