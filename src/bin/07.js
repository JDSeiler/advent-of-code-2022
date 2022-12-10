import { readFileSync } from "fs";

const getInput = (sample = false) => {
    return sample ?
        readFileSync("07-sample.txt", { encoding: "utf-8" }) :
        readFileSync("07.txt", { encoding: "utf-8" });

}

const newDir = (name) => {
    return {
        name,
        parent: null,
        totalSize: 0,
        files: [],
        children: [],
    }
}

const addChild = (parentDirectory, childDirectory) => {
    childDirectory.parent = parentDirectory;
    parentDirectory.children.push(childDirectory);
}

const addFile = (cwd, file) => {
    cwd.files.push(file);
}

const cdIn = (cwd, destName) => {
    return cwd.children.find(dir => dir.name === destName);
}

const cdUp = (cwd) => {
    return cwd.parent;
}

const calculateTotalSizes = (directory) => {
    if (directory.children.length === 0) {
        directory.totalSize += directory.files.reduce(
            (acc, file) => acc + file.size,
            0
        );
        return directory.totalSize;
    } else {
        directory.totalSize += directory.files.reduce(
            (acc, file) => acc + file.size,
            0
        );
        directory.totalSize += directory.children.reduce(
            (acc, dir) => acc + calculateTotalSizes(dir),
            0
        );
        return directory.totalSize;
    }
}

const buildTreeFromInput = (input) => {
    const rootDir = newDir("/");
    let cwd = rootDir;
    input.split("\n").forEach(line => {
        if (line.startsWith("$ cd")) {
            const [_, __, destName] = line.split(" ");
    
            if (destName === "/") {
                // do nothing
            } else if (destName === "..") {
                cwd = cdUp(cwd);
            } else {
                cwd = cdIn(cwd, destName);
            }
        } else if (line.startsWith("$ ls")) {
            // do nothing
        } else {
            if (line.startsWith("d")) {
                const [_, dirName] = line.split(" ");
                addChild(cwd, newDir(dirName))
            } else {
                const [size, name] = line.split(" ");
                const file = { name, size: Number(size) };
                addFile(cwd, file);
            }
        }
    });
    // mutates rootDir
    calculateTotalSizes(rootDir);

    return rootDir;
}

const getDirectoriesUnderSize = (cwd, sizeCap = 100000) => {
    if (cwd.children.length === 0) {
        if (cwd.totalSize <= sizeCap) {
            return [ cwd ];
        }
        return [];
    } else {
        const dirs = [];
        if (cwd.totalSize <= sizeCap) {
            dirs.push(cwd);
        }
        return dirs.concat(
            cwd.children.flatMap(dir => getDirectoriesUnderSize(dir, sizeCap))
        );
    }
}


const partOne = () => {
    const input = getInput();
    const fileSystem = buildTreeFromInput(input); 
    
    const smallDirs = getDirectoriesUnderSize(fileSystem);
    
    const answer = smallDirs.reduce((acc, item) => acc + item.totalSize, 0);
    console.log(answer);
}

const getAllDirectories = (cwd) => {
    if (cwd.children.length === 0) {
        return [cwd];
    } else {
        return [cwd].concat(cwd.children.flatMap(dir => getAllDirectories(dir)));
    }
}

const partTwo = () => {
    const DISK_SPACE = 70000000;
    const NEEDED_SPACE = 30000000;

    const input = getInput();
    const fileSystem = buildTreeFromInput(input);

    const unusedSpace = DISK_SPACE - fileSystem.totalSize;
    const SPACE_TO_FREE = NEEDED_SPACE - unusedSpace;

    const allDirectories = getAllDirectories(fileSystem);
    // Sort smallest to largest
    allDirectories.sort((a, b) => a.totalSize - b.totalSize);
    for (const dir of allDirectories) {
        if (dir.totalSize > SPACE_TO_FREE) {
            console.log(dir.totalSize);
            return;
        }
    }
    console.log("No answer found :(");
}
