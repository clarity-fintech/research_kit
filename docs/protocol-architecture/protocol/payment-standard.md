# CLARITY PAY Payment Standard

Unified payment primitive for CLRTY L1 — validate, route, settle.

## Client

```javascript
await clarityPay({ to: 'clrty1merchant', amount: 100, currency: 'CLRTY' });
```

Frontend: [`frontend/tools/clarity-pay.html`](../../frontend/tools/clarity-pay.html)

## Commerce Kit

```javascript
// Checkout component (scaffold)
createCheckout({ amount: 100, token: 'CLRTY' });
```

Frontend: [`frontend/tools/commerce-kit.html`](../../frontend/tools/commerce-kit.html)

## Settlement backend

| Step | Module |
|------|--------|
| Validate | `settlement/commit_payment.rs` |
| KYC gate | `kyc_webhook.rs`, attestation blobs |
| Treasury | `GET /v1/compliance/treasury` |
| Confirm | `POST /v1/compliance/deposit/confirm` |

## Payment tooling modes (scaffold)

- Recurring
- Split payments
- Delegated execution

See [`frontend/tools/payments-tooling.html`](../../frontend/tools/payments-tooling.html)

## Institutional path

OTC/MIRRA blocks use settlement gatekeeper — not public mempool. See [`docs/investor/settlement_gatekeeper.md`](../investor/settlement_gatekeeper.md).
