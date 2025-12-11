#!/bin/bash
input=day11.dot
for layout in patchwork dot neato sfdp circo twopi osage fdp
do
	dot -K $layout -T svg $input > $input.$layout.svg
done

