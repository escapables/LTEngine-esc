---
summary: 'Primary execution plan for /translate_file endpoint implementation.'
read_when:
  - Implementing file translation feature.
  - Tracking translate_file development progress.
---

> **MAINTAINER-ONLY DOCUMENT**
> This is an ephemeral internal document for tracking multi-step workstreams.
> It is temporary state for active development sessions.
> End users and contributors should refer to ROADMAP.md for planned features.

# PRIMARY TODO: Implement /translate_file Endpoint

## Objective

- Implement LibreTranslate-compatible `/translate_file` endpoint for document translation.
- Enable users to upload files and receive translated documents.

## Interface / Artifact Contract

### Request

- **Method**: POST `/translate_file`
- **Content-Type**: `multipart/form-data`
- **Parameters**:
  - `file` (required): File to translate
  - `source` (required): Source language code or "auto"
  - `target` (required): Target language code
  - `api_key` (optional): API key if server requires one

### Response

- **Success**: JSON with `translatedFileUrl` field pointing to downloadable translated file
- **Error**: JSON with `error` field and appropriate HTTP status

### Frontend Settings Update

- `filesTranslation`: `true`
- `supportedFilesFormat`: `[".txt"]` (initially)

## Required Scenarios

1. **Happy path**: Upload `.txt` file, receive translated file URL, download translated content.
2. **Auto-detect source**: Upload file with `source=auto`, detect language, translate.
3. **Error handling**: Invalid file type, missing parameters, file too large.
4. **API key validation**: Enforce key if server configured with one.

## Execution Plan

### Phase 1: Core Implementation - DONE

1. [x] Add file upload handling dependencies if needed
   - Verified `actix-multipart` covers file field handling
   - Added `uuid` crate for download ID generation

2. [x] Create request/response types for file translation
   - Implemented inline in handler using `Multipart` trait
   - `TranslateFileResponse` struct with `translatedFileUrl`

3. [x] Implement file upload processing
   - Extract file content from multipart form
   - Validate file type (`.txt` only initially)
   - Enforce file size limits (10MB)

4. [x] Implement translation logic
   - Read file content as UTF-8 text
   - Reuse existing `translate` logic for text translation
   - Handle encoding errors gracefully

5. [x] Implement file serving mechanism
   - Generate unique download IDs via UUID v4
   - Store translated content in-memory with `FileStore` struct
   - Created `/download/{id}` endpoint to serve translated files (1-hour TTL)

6. [x] Update frontend settings
   - Set `filesTranslation: true`
   - Set `supportedFilesFormat: [".txt"]`

### Phase 2: Polish and Testing

7. [ ] Add error handling
   - Invalid file type (415 Unsupported Media Type)
   - File too large (413 Payload Too Large)
   - Encoding errors (400 Bad Request)
   - Missing parameters (400 Bad Request)

8. [ ] Add integration tests
   - Test file upload and translation
   - Test auto-detect source language
   - Test error scenarios

9. [ ] Replace line-based file limit with size-based limit
   - Remove any line-count rejection path in `translate_file`
   - Enforce byte-size limit only (configurable `file_size_limit`)
   - Increase default file size limit from current baseline if needed for real documents

10. [ ] Update documentation
   - Update `ARCHITECTURE.md` API surface
   - Update `PORTABLE_APP.md` with supported formats
   - Update `README.md` feature list

## Evidence

- `curl -F "file=@test.txt" -F "source=en" -F "target=es" http://localhost:5050/translate_file` returns JSON with `translatedFileUrl`
- Download URL returns translated file content
- Frontend file translation UI works end-to-end
- All tests pass: `cargo test`

## Acceptance Criteria

- [x] `/translate_file` accepts `.txt` file uploads
- [x] Returns downloadable translated file URL
- [x] Supports `source=auto` for language detection
- [x] Validates API key when configured
- [x] Returns appropriate errors for invalid inputs
- [x] Frontend settings reflect file translation capability
- [ ] Documentation updated to reflect new feature (ARCHITECTURE.md, PORTABLE_APP.md, README.md)

## Defaults Chosen

- **Initial file format**: `.txt` only (plain text). HTML, DOCX, PDF can be added later.
- **File storage**: In-memory with UUID-based download IDs. Simpler than temp files, suitable for single-instance deployment.
- **Size limit**: Use size-based validation only for uploaded files (no line-count limit); keep `file_size_limit` CLI arg as the source of truth (default 10MB, can be raised).
- **Download expiry**: Translated files available for 1 hour (cleanup via background task or on-demand).

## Architecture Diagram

```mermaid
sequenceDiagram
    participant Client
    participant Server as actix-web
    participant Handler as translate_file
    participant LLM
    participant Store as FileStore

    Client->>Server: POST /translate_file with file
    Server->>Handler: Multipart form data
    Handler->>Handler: Validate file type and size
    Handler->>Handler: Extract text content
    Handler->>LLM: Translate text
    LLM-->>Handler: Translated text
    Handler->>Store: Store with UUID key
    Handler-->>Client: JSON with translatedFileUrl
    Client->>Server: GET /download/{uuid}
    Server->>Store: Retrieve content
    Store-->>Client: Translated file
```

## Key Files to Modify

| File | Changes |
|------|---------|
| `ltengine/src/main.rs` | Add `translate_file` handler, `download` endpoint, file store |
| `ltengine/Cargo.toml` | Add `tempfile` or `uuid` crate if needed |
| `.kilocode/rules/ARCHITECTURE.md` | Update API surface documentation |
| `docs/PORTABLE_APP.md` | Update supported formats list |
| `README.md` | Update feature list |
