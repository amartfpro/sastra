# ADR 0002 · Formato de proyecto `.sastra` sobre SQLite

**Estado:** aceptada · 11/11/2026

## Decisión

- Un proyecto es un único archivo SQLite con extensión `.sastra` y `application_id` = `0x53415354` («SAST»),
  para reconocerlo aunque cambie de nombre.
- El esquema se versiona con `PRAGMA user_version` y migraciones que solo se añaden, nunca se editan.
  Un archivo de una versión más nueva se rechaza con un mensaje claro.
- Diario `DELETE` y `synchronous = FULL`: no quedan archivos auxiliares junto al proyecto, lo que es más
  seguro en carpetas sincronizadas con la nube.
- Cada cambio se guarda al momento, en una transacción que también anota el antes y el después en la tabla
  `changes`. De ahí saldrán el deshacer, el historial de versiones de la ficha y la sincronización.
- Identificadores UUID v7; fechas en milisegundos desde 1970 (UTC).
- «Guardar una copia» usa `VACUUM INTO` para producir un archivo limpio y compacto.

## Pendiente

- Los recursos pesados (PSD, AI, fotos) irán en una carpeta junto al proyecto, identificados por su huella
  (hash), para que las versiones no los dupliquen (entrega 1.2).
- Copias de seguridad automáticas periódicas.
