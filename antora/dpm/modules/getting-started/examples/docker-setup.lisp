; Docker environment setup example
; Usage: dpm --file docker-setup.lisp

; Set project name from current directory
(rust-env-set "PROJECT_NAME" (rust-path-filename (rust-env-current-dir)))

; Set user/group IDs (cross-platform)
(rust-env-set "HOST_UID" (rust-process-output "id" "-u"))
(rust-env-set "HOST_GID" (rust-process-output "id" "-g"))

; Set Docker BuildKit
(rust-env-set "DOCKER_BUILDKIT" "1")

; Print setup info
(print "Project: " (rust-env-var "PROJECT_NAME"))
(print "UID: " (rust-env-var "HOST_UID"))
(print "GID: " (rust-env-var "HOST_GID"))

; Start Docker Compose services
(docker "compose" "up" "-d")
