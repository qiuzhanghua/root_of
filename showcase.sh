#!/bin/bash
pushd `pwd` > /dev/null
cd `dirname $0`
exec_path=$("./target/release/root_of")
if [ "$?" != "0" ]; then
  echo "Can't find executed file path."
  exit 1
fi
echo "$exec_path"
popd > /dev/null
