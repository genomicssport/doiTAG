#!/usr/bin/bash

# Author Gaurav Sablok
# Instytut Chemii Bioorganicznej
# Polskiej Akademii Nauk
# ul. Noskowskiego 12/14 | 61-704, Poznań
# Date: 2025-22-8

cat fastafile | while read line; do echo md5sum $line >> estimatetag.txt; done
