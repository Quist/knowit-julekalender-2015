<?php
if (count($argv) < 2) {
    echo "Usage: $argv[0] [file]\n";
    exit;
}

$file = $argv[1];
if (!file_exists($file)) {
    echo "File does not exist: $file\n";
    exit;
}

$dna_sample = file_get_contents($file);

$adenin_count = substr_count($dna_sample, "A");
$cytosin_count = substr_count($dna_sample, "C");
$guanin_count = substr_count($dna_sample, "G");
$tymin_count = substr_count($dna_sample, "T");

echo "A$adenin_count,C$cytosin_count,G$guanin_count,T$tymin_count\n";
?>
