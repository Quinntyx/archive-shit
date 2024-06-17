# archive-shit
`archive-shit` (or `.shit`) is a file archive format that lets you `archive` your `shit` similar to `.tar.xvf` or `.tar.gz`.

The primary benefit of using `archive-shit` is that the name is funny. 

I wrote `archive-shit` for fun, and it probably won't get many feature updates anytime soon. Also, there may be breaking changes. 
I don't recommend you use this for anything important, but if you want to look at the code and figure out how it works, be my guest. 

# How It Works
The implementation of `archive-shit` is pretty much the simplest implementation possible. It does the following things: 
1. Flatten all paths and folders passed to get a list of files to archive
2. Create a list of serializable objects, one for each file, containing the path to the file relative to the directory the tool is run in and its contents as a `Vec<u8>`
3. Serialize the list of objects using `serde` and `bincode`, to produce a `Vec<u8>` representation of the archive
4. Compress the resulting `Vec<u8>` using various compression techniques (currently, `flate3` and `lzma/xz` with various settings) to find the one with the best ratio.
5. Write the resulting compressed `Vec<u8>` to the passed file location, usually a `.shit` file (but not required to be). 

# Todo/Roadmap
- [x] Implement a post-compression header struct appended to the front to store information like compression schema, in case the algorithm ever changes from `flate3` in the future, to enable opening all `.shit` archives even if originally archived using older version of the tool
    - [-] switch to `VecDeque<T>` representation? will not be implemented
- [ ] Implement a custom compression algorithm ?
- [ ] Remove Herobrine
- [ ] update `libshit` to use less `.except(err)` and return more `Result<T, E>` structs, and do more of the error handling in the `makeshit` and `breakshit` CLI tools, to make `libshit` more accessible for embedding in other tools in the future
