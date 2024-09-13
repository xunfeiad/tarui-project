import { writable } from 'svelte/store';
import {tweened} from "svelte/motion";
import { cubicOut } from 'svelte/easing';

export const progress = tweened(0, {
    duration: 400,
    easing: cubicOut
});

export const loadingState = writable(false);

