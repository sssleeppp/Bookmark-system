# Bookmark System development commands

default:
    @just --list

# ─────────────────────────────────────────────
#  Build & Server
# ─────────────────────────────────────────────

build:
    cd backend && cargo build
    cd frontend && bun install

start-backend:
    #!/usr/bin/env bash
    cd backend
    nohup cargo run 1>/tmp/bookmark-server.log 2>&1 &
    sleep 1
    echo "Backend: http://localhost:8989"

start-frontend:
    #!/usr/bin/env bash
    cd frontend
    test -d node_modules || bun install
    nohup bun run dev 1>/tmp/bookmark-frontend.log 2>&1 &
    sleep 1
    echo "Frontend: http://localhost:5173"

start-all: start-backend start-frontend

stop:
    -pkill -f "bookmark-backend"
    -pkill -f "vite"

fmt:
    cd backend && cargo fmt
    cd frontend && bunx prettier --write .
    alejandra flake.nix

check:
    cd backend && cargo clippy
    cd frontend && bunx prettier --check .

fix:
    cd backend && cargo clippy --fix --allow-dirty
    cd frontend && bunx prettier --write .

# ─────────────────────────────────────────────
#  API tests (requires "just start-backend")
# ─────────────────────────────────────────────

BASE := "http://localhost:8989"

test-all: test-login test-login-wrong test-register test-category-crud test-bookmark-crud test-bookmark-export-json test-bookmark-export-html test-bookmark-import-json test-bookmark-import-html test-category-tree-delete

# POST /user/login — correct password
test-login:
    #!/usr/bin/env bash
    curl -s -X POST {{ BASE }}/user/login \
    	-H "Content-Type: application/json" \
    	-d '{"username":"admin","password":"123456"}' | jq

# POST /user/login — wrong password
test-login-wrong:
    #!/usr/bin/env bash
    curl -s -X POST {{ BASE }}/user/login \
    	-H "Content-Type: application/json" \
    	-d '{"username":"admin","password":"wrong"}' | jq

# POST /user/register — register new user
test-register:
    #!/usr/bin/env bash
    curl -s -X POST {{ BASE }}/user/register \
    	-H "Content-Type: application/json" \
    	-d '{"username":"test","password":"123456"}' | jq

# Category CRUD: add → list → update → delete
test-category-crud:
    #!/usr/bin/env bash
    set -e
    echo "=== Add category ==="
    curl -s -X POST {{ BASE }}/category/add \
    	-H "Content-Type: application/json" \
    	-d '{"name":"Test","userId":1,"parentId":null,"sortOrder":0}' | jq
    echo ""
    echo "=== List categories ==="
    curl -s "{{ BASE }}/category/list?userId=1" | jq
    echo ""
    echo "=== Batch update ==="
    curl -s -X POST {{ BASE }}/category/batchUpdate \
    	-H "Content-Type: application/json" \
    	-d '[{"id":1,"name":"TestRenamed","userId":1,"parentId":null,"sortOrder":99}]' | jq
    echo ""
    echo "=== List after update ==="
    curl -s "{{ BASE }}/category/list?userId=1" | jq
    echo ""
    echo "=== Delete category ==="
    curl -s -X POST {{ BASE }}/category/delete \
    	-H "Content-Type: application/json" \
    	-d '{"id":1}' | jq

# Bookmark CRUD: add → list → delete
test-bookmark-crud:
    #!/usr/bin/env bash
    set -e
    echo "=== Add bookmark ==="
    curl -s -X POST {{ BASE }}/bookmark/add \
    	-H "Content-Type: application/json" \
    	-d '{"title":"GitHub","url":"https://github.com","userId":1}' | jq
    echo ""
    echo "=== List bookmarks ==="
    curl -s "{{ BASE }}/bookmark/list?userId=1" | jq
    echo ""
    echo "=== Delete bookmark ==="
    curl -s -X POST {{ BASE }}/bookmark/delete \
    	-H "Content-Type: application/json" \
    	-d '{"id":1}' | jq

# GET /bookmark/export?format=json
test-bookmark-export-json:
    #!/usr/bin/env bash
    curl -s "{{ BASE }}/bookmark/export?userId=1&format=json" | head -c 500
    echo ""

# GET /bookmark/export?format=html
test-bookmark-export-html:
    #!/usr/bin/env bash
    curl -s "{{ BASE }}/bookmark/export?userId=1&format=html" | head -c 500
    echo ""

# POST /bookmark/import — JSON import
test-bookmark-import-json:
    #!/usr/bin/env bash
    set -e
    echo '{"bookmarks":[{"title":"Imported","url":"https://example.com"}],"categories":[]}' > /tmp/test_import.json
    echo "=== Import JSON ==="
    curl -s -X POST {{ BASE }}/bookmark/import \
    	-F "userId=1" \
    	-F "format=json" \
    	-F "file=@/tmp/test_import.json" | jq
    rm -f /tmp/test_import.json

# POST /bookmark/import — HTML import
test-bookmark-import-html:
    #!/usr/bin/env bash
    set -e
    printf '%s\n' \
        '<!DOCTYPE NETSCAPE-Bookmark-file-1>' \
        '<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">' \
        '<TITLE>Bookmarks</TITLE>' \
        '<H1>书签菜单</H1>' \
        '<DL><p>' \
        '    <DT><H3>Folder</H3>' \
        '    <DL><p>' \
        '        <DT><A HREF="https://nested.example.com">Nested Bookmark</A>' \
        '    </DL><p>' \
        '    <DT><A HREF="https://root.example.com">Root Bookmark</A>' \
        '</DL><p>' \
        > /tmp/test_import.html
    echo "=== Import HTML ==="
    curl -s -X POST {{ BASE }}/bookmark/import \
    	-F "userId=1" \
    	-F "format=html" \
    	-F "file=@/tmp/test_import.html" | jq
    rm -f /tmp/test_import.html

# POST /category/delete — recursive tree delete
test-category-tree-delete:
    #!/usr/bin/env bash
    set -e
    echo "=== Create parent category ==="
    PARENT=$(curl -s -X POST {{ BASE }}/category/add \
    	-H "Content-Type: application/json" \
    	-d '{"name":"Folder","userId":1,"sortOrder":0}' | jq -r .data.id)
    echo "Parent ID: $PARENT"
    echo "=== Create child category ==="
    CHILD=$(curl -s -X POST {{ BASE }}/category/add \
    	-H "Content-Type: application/json" \
    	-d "{\"name\":\"SubFolder\",\"userId\":1,\"parentId\":$PARENT,\"sortOrder\":0}" | jq -r .data.id)
    echo "Child ID: $CHILD"
    echo "=== Add bookmark in child ==="
    curl -s -X POST {{ BASE }}/bookmark/add \
    	-H "Content-Type: application/json" \
    	-d "{\"title\":\"InFolder\",\"url\":\"https://foo.bar\",\"userId\":1,\"categoryId\":$CHILD}" | jq
    echo ""
    echo "=== Delete parent (recursive) ==="
    curl -s -X POST {{ BASE }}/category/delete \
    	-H "Content-Type: application/json" \
    	-d "{\"id\":$PARENT}" | jq
    echo ""
    echo "=== Verify categories empty ==="
    curl -s "{{ BASE }}/category/list?userId=1" | jq '.data | length'
    echo "=== Verify bookmarks empty ==="
    curl -s "{{ BASE }}/bookmark/list?userId=1" | jq '.data | length'
