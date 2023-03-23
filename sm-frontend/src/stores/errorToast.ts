import {ToastOptions} from "vue-toastification/dist/types/types";
import {TYPE, useToast} from "vue-toastification";

export function displayToast(toastType: string, toastMessage: string) {
    const toastOptions: ToastOptions = {
        timeout: 10000,
        closeOnClick: true,
        pauseOnFocusLoss: true,
        pauseOnHover: true,
        draggable: true,
        draggablePercent: 0.6,
        showCloseButtonOnHover: true,
        hideProgressBar: false,
        closeButton: "button",
        icon: true,
        rtl: false,
    };
    const toast = useToast();
    switch (toastType) {
        case "error":
            toastOptions.type = TYPE.ERROR;
            break;
        case "success":
            toastOptions.type = TYPE.SUCCESS;
            break;
        default:
            toastOptions.type = TYPE.INFO;
    }
    toast(toastMessage, toastOptions);
}
