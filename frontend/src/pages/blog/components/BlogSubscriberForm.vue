<template>
    <v-sheet>
        <v-tabs v-model="tab" color="primary" grow>
            <v-tab value="one">Subscribe</v-tab>
            <v-tab value="two">Unsubscribe</v-tab>
        </v-tabs>
        <v-divider></v-divider>
        <v-tabs-window v-model="tab">
            <v-tabs-window-item value="one">
                <div class="form-wrap">
                    <v-form @submit.prevent="handleSubscribeClick" class="form">
                        <div class="field">
                            <label for="sub-email">Email</label>
                            <v-text-field id="sub-email" v-model="subscribeEmail" :rules="emailRules" variant="outlined"
                                density="compact" placeholder="you@example.com" hide-details="auto" />
                        </div>
                        <button type="submit" class="btn-primary">Subscribe</button>
                    </v-form>
                </div>
            </v-tabs-window-item>
            <v-tabs-window-item value="two">
                <div class="form-wrap">
                    <v-form @submit.prevent="handleUnsubscribeClick" class="form">
                        <div class="field">
                            <label for="unsub-email">Email</label>
                            <v-text-field id="unsub-email" v-model="unsubscribeEmail" :rules="emailRules"
                                variant="outlined" density="compact" placeholder="you@example.com"
                                hide-details="auto" />
                        </div>
                        <button type="submit" class="btn-primary">Unsubscribe</button>
                    </v-form>
                </div>
            </v-tabs-window-item>
        </v-tabs-window>
    </v-sheet>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { isValidEmail } from '../../../helpers/utilities/regex';
import type { Ref } from 'vue';
import { apiFetch } from '../../../api';
import { showSnackBar } from '../../../components/subjects/snack-bar-subject';

const subscribeEmail: Ref<string, string> = ref('');
const unsubscribeEmail: Ref<string, string> = ref('');
const tab: Ref<string, string> = ref('one');

const emailRules = [
    (value: string) => {
        if (value && isValidEmail(value)) {
            return true;
        }
        return 'You must enter a valid email.';
    },
]

const handleUnsubscribeClick = async () => {
    if (!isValidEmail(unsubscribeEmail.value)) {
        return;
    }

    try {
        const res = await apiFetch('/api/blog/unsubscribe', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: unsubscribeEmail.value })
        });
        if (!res.ok) {
            throw new Error('Unsubscription failed');
        }
        unsubscribeEmail.value = '';
        showSnackBar('Check your email to confirm unsubscription.', 'success');
    } catch (e: any) {
        showSnackBar(e.message, 'error');
    }
}

const handleSubscribeClick = async () => {
    if (!isValidEmail(subscribeEmail.value)) {
        return;
    }

    try {
        const res = await apiFetch('/api/blog/subscribe', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: subscribeEmail.value })
        });
        if (!res.ok) {
            throw new Error('Subscription failed');
        }
        subscribeEmail.value = '';
        showSnackBar('Check your email to confirm your subscription.', 'success');
    } catch (e: any) {
        showSnackBar(e.message, 'error');
    }
}
</script>

<style scoped>
.form-wrap {
    padding: 1.5rem;
}

.form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
}

.field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
}

label {
    font-size: 0.85rem;
    color: var(--text-muted);
    font-family: var(--mono);
}

.btn-primary {
    border: none;
    cursor: pointer;
    font-family: inherit;
    font-size: 0.95rem;
}

.btn-primary:hover {
    background: var(--accent-hover);
}
</style>
