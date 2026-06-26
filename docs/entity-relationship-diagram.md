```mermaid
erDiagram
    CONDITIONS {
        id UUID PK
        input BLOB
        output TEXT UK
        profile_id UUID UK, FK
    }

    PROFILES {
        id UUID PK
        kind TEXT
        name TEXT UK
    }

    CONDITIONS }|--|| PROFILES: "belongs to"
```