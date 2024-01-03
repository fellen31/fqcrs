# fqcrs

Output read ID, length and quality from a fastq-file. 

## Installation

```bash
cargo install --git https://github.com/fellen31/fqcrs.git
```

## Usage 

```bash
cat reads.fq | fqcrs > out.tsv
```

## Example output 

(Read ID, length, quality)
```
8093c4a5-6e95-4354-a070-4c456a515081    170     10.957026
a10bf756-0388-451d-a531-43288cd148f4    2394    14.637847
aded3d78-f58b-4413-9f89-ebca9e274765    1179    12.717719
```

