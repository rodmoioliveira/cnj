#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace

comments-tidy() {
  grep -rE --null --include=\*.rs '‐' -l | xargs -r -0 sed -i -E 's@‐@-@g'
  grep -rE --null --include=\*.rs '\w- {1,5}' -l | xargs -r -0 sed -i -E 's@(\w)- {1,8}@\1@g'
  grep -rE --null --include=\*.rs "’" -l | xargs -r -0 sed -i -E "s@’@'@g"
  grep -rE --null --include=\*.rs "”" -l | xargs -r -0 sed -i -E 's@”@"@g'
  grep -rE --null --include=\*.rs "“" -l | xargs -r -0 sed -i -E 's@“@"@g'
  grep -rE --null --include=\*.rs "ﬀ" -l | xargs -r -0 sed -i -E 's@ﬀ@ff@g'
  grep -rE --null --include=\*.rs "ﬃ" -l | xargs -r -0 sed -i -E 's@ﬃ@ffi@g'
  grep -rE --null --include=\*.rs "ﬁ" -l | xargs -r -0 sed -i -E 's@ﬁ@fi@g'
  grep -rE --null --include=\*.rs "→" -l | xargs -r -0 sed -i -E 's@→@->@g'
}

main() {
  comments-tidy
}

main
