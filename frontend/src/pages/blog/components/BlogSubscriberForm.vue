<template>
    <v-sheet>
        <v-tabs v-model="tab" color="primary" grow>
            <v-tab value="one">Subscribe</v-tab>
            <v-tab value="two">Unsubscribe</v-tab>
        </v-tabs>
        <v-divider></v-divider>
        <v-tabs-window v-model="tab">
            <v-tabs-window-item value="one">
                <v-sheet class="pa-5">
                    <v-form @submit.prevent="handleSubscribeClick">
                        <v-text-field v-model="subscribeEmail" :rules="emailRules" label="Email"></v-text-field>
                        <v-btn class="mt-2" @click="handleSubscribeClick" block>Subscribe</v-btn>
                    </v-form>
                </v-sheet>
            </v-tabs-window-item>
            <v-tabs-window-item value="two">
                <v-sheet class="pa-5">
                    <v-form @submit.prevent="handleUnsubscribeClick">
                        <v-text-field v-model="unsubscribeEmail" :rules="emailRules" label="Email"></v-text-field>
                        <v-btn class="mt-2" @click="handleUnsubscribeClick" block>Unsubscribe</v-btn>
                    </v-form>
                </v-sheet>
            </v-tabs-window-item>
        </v-tabs-window>
    </v-sheet>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { isValidEmail } from '../../../helpers/utilities/regex';
import type { Ref } from 'vue';
import { apiFetch } from '../../../api';

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
    } catch (e: any) {
        alert(e.message);
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
    } catch (e: any) {
        alert(e.message);
    }
}
</script>
