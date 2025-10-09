#!/bin/bash
set -e

JUNIT_FILE=test_report.xml
SRC_DIR=./src
ALLURE_RESULTS=allure-results
ALLURE_REPORT=allure-report

cd "$SRC_DIR"
cargo +nightly test --no-fail-fast -p business_logic -p data_access -p api --tests \
  -- -Z unstable-options --report-time --format json --skip clickhouse \
  | cargo2junit > "$JUNIT_FILE"
cd ..

rm -rf "$ALLURE_RESULTS"
mkdir -p "$ALLURE_RESULTS"
cp "$SRC_DIR/$JUNIT_FILE" "$ALLURE_RESULTS/"

count=$(xmlstarlet sel -t -v "count(/testsuites/testsuite)" "$SRC_DIR/$JUNIT_FILE")

for i in $(seq $count -1 1); do
  classname=$(xmlstarlet sel -t -v "/testsuites/testsuite[$i]/testcase[1]/@classname" "$SRC_DIR/$JUNIT_FILE" || echo "")
  if [ -n "$classname" ]; then
    xmlstarlet ed -L \
      -u "/testsuites/testsuite[$i]/@name" \
      -v "$classname" \
      "$ALLURE_RESULTS/$JUNIT_FILE"
  else
    xmlstarlet ed -L \
      -d "/testsuites/testsuite[$i]" \
      "$ALLURE_RESULTS/$JUNIT_FILE"
  fi
done

rm "$SRC_DIR/$JUNIT_FILE"

if [ -d "$ALLURE_REPORT/history" ]; then
  cp -r "$ALLURE_REPORT/history" "$ALLURE_RESULTS/"
fi

echo "generate allure"
allure generate "$ALLURE_RESULTS" -o "$ALLURE_REPORT" --clean
echo "allure generated"
