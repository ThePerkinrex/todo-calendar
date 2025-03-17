export class LocalDate extends Date {
	constructor(...args) {
		if (args.length == 1 && args[0] instanceof UTCDate) {
			super(args[0].toLocal())
		}else{
			super(...args)
		}
	}

	toUTC() {
		return new UTCDate(this.getTime() - this.getTimezoneOffset() * 60 * 1000)
	}

	toISOString() {
		return this.toUTC().toISOString()
	}

	toLocalISOString() {
		return super.toISOString()
	}
}

export class UTCDate extends Date {
	constructor(...args) {
		if (args.length == 1 && args[0] instanceof LocalDate) {
			super(args[0].toUTC())
		}else{
			super(...args)
		}
	}

	toLocal() {
		return new LocalDate(this.getTime() + this.getTimezoneOffset() * 60 * 1000)
	}
}