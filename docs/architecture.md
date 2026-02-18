# Arquitectura

## Inmutabilidad en Dominio (Checklist)
- [ ] Sin &mut self en entidades de dominio: todas las transiciones usan self -> DomainResult<Self> o &self -> DomainResult<Self> (preferido: consumir self).
- [ ] Transiciones puras: métodos como mark_done, edit_title no hacen I/O ni conocen repositorios.
- [ ] Validación dentro del dominio: cada transición valida invariantes y devuelve DomainError cuando corresponde.
- [ ] Sin mutación externa de campos: campos privados + getters; cambios solo vía métodos de dominio.
- [ ] Timestamps coherentes: created_at se preserva; modified_at se actualiza solo en transiciones exitosas.
- [ ] Consistencia de estilo: no mezclar en la misma entidad mutación in-place y estilo inmutable.
- [ ] Application orquesta, Domain decide: casos de uso coordinan repo + dominio; reglas de cambio viven en domain.
- [ ] Repositorios guardan snapshots completos: save(updated_task) persiste la nueva versión de entidad.
- [ ] Errores por capa: DomainError para reglas; RepoError para infraestructura; no mezclar.
- [ ] Tests obligatorios de transiciones: caso feliz + caso inválido por cada método de dominio.
  Convención recomendada

- Entidades: métodos de transición con firma self -> DomainResult<Self>.
- Getters: retornos por copia o referencia (Uuid, TaskStatus, &str, &DateTime<Utc>).
- Casos de uso: find -> transition -> save.